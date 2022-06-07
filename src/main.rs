use sdl2::event::Event;
use std::fs;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Rust-Skia OpenGL / SDL2 Demo", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    window.gl_swap_window();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'main;
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let mut draw_id = 0;
        unsafe {
            gl::GetIntegerv(gl::DRAW_FRAMEBUFFER_BINDING, &mut draw_id);
        }

        {
            let mut surface = draw_line_with_skia(draw_id, window.size());
            let image = surface.image_snapshot();
            let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
            let bytes = data.as_bytes();
            fs::write("snapshot.png", bytes).expect("failed to write file");
        }

        window.gl_swap_window();
    }
}

use skia_safe::{
    gpu, Color, ColorType, EncodedImageFormat, Paint, PixelGeometry, Surface, SurfaceProps,
    SurfacePropsFlags,
};

fn draw_line_with_skia(buffer_id: i32, (width, height): (u32, u32)) -> Surface {
    let interface = gpu::gl::Interface::new_native();
    let context = gpu::DirectContext::new_gl(interface.unwrap(), None);
    let mut ctx = context.unwrap();
    let mut frame_buffer = gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);
    frame_buffer.format = 0x8058; //GR_GL_RGBA8 (https://github.com/google/skia/blob/master/src/gpu/gl/GrGLDefines.h#L511)
    let target =
        gpu::BackendRenderTarget::new_gl((width as i32, height as i32), Some(0), 8, frame_buffer);
    let surface_props = SurfaceProps::new(SurfacePropsFlags::default(), PixelGeometry::Unknown);
    let surface_holder = Surface::from_backend_render_target(
        &mut ctx,
        &target,
        gpu::SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        Some(&surface_props),
    );
    let mut surface = surface_holder.unwrap();
    {
        let canvas = surface.canvas();
        let mut paint = Paint::default();
        paint.set_color(Color::new(0xffff0000));
        canvas.draw_line((0, 0), (100, 100), &paint);
    }
    surface
}
