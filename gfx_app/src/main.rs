#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use std::ffi::CString;
use gfx::traits::{Stream, ToIndexSlice, ToSlice, FactoryExt};

gfx_vertex!( Vertex {
    position@ pos: [f32; 3],
    color@ color: [f32; 3],
});

// Shader sources
static VERTEX_SOURCE: &'static str =
   "#version 130\n\

    in vec3 position;\n\
    in vec3 color;\n\
    out vec4 v_color;\n\

    void main() {\n\
       v_color = vec4(color, 1.0);\n\
       gl_Position = vec4(position, 1.0);\n\
    }";

static FRAGMENT_SOURCE: &'static str =
   "#version 130\n\

    in vec4 v_color;\n\
    out vec4 color;\n\

    void main() {\n\
       color = v_color;\n\
    }";

pub fn main() {
    let (mut stream, mut device, mut factory) = gfx_window_glutin::init(
        glutin::Window::new().unwrap());

    stream.out.window.set_title("GFX App");

    let vertex_data = [
        Vertex { pos: [ -0.5, -0.5, 0.0 ], color: [1.0, 0.0, 0.0] },
        Vertex { pos: [  0.5, -0.5, 0.0 ], color: [0.0, 1.0, 0.0] },
        Vertex { pos: [  0.0,  0.5, 0.0 ], color: [0.0, 0.0, 1.0] },
    ];
    let mesh = factory.create_mesh(&vertex_data);
    let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);

    let program = {
        let vs = gfx::ShaderSource {
            glsl_120: Some(VERTEX_SOURCE.as_bytes()),
            glsl_150: Some(VERTEX_SOURCE.as_bytes()),
            .. gfx::ShaderSource::empty()
        };
        let fs = gfx::ShaderSource {
            glsl_120: Some(FRAGMENT_SOURCE.as_bytes()),
            glsl_150: Some(FRAGMENT_SOURCE.as_bytes()),
            .. gfx::ShaderSource::empty()
        };
        factory.link_program_source(vs, fs).unwrap()
    };
    let state = gfx::DrawState::new();

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

        stream.draw(&gfx::batch::bind(&state, &mesh, slice.clone(), &program, &None))
              .unwrap();

        stream.present(&mut device);
    }
}
