#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::{Stream, ToIndexSlice, ToSlice, FactoryExt};

gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 3],
    a_Color@ color: [f32; 3],
});

pub fn main() {
    let (mut stream, mut device, mut factory) = gfx_window_glutin::init(
        glutin::Window::new().unwrap());

    stream.out.window.set_title("GFX App");

    'main: loop {
        // Quit when esc key is pressed
        for event in stream.out.window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) => break 'main,
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }

        stream.clear(gfx::ClearData {
            color: [0.5, 0.5, 0.5, 1.0],
            depth: 1.0,
            stencil: 0
        });

        stream.present(&mut device);
    }
}
