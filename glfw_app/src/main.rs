extern crate glfw;
extern crate gl;

use std::sync::mpsc::Receiver;
use std::mem;
use std::ptr;
use std::ffi;
use glfw::{Action, Context, Key};
use gl::types::*;

// Global static variables

// Shader sources
static vertex_src: &'static str =
   "#version 150\n\
    in vec3 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 1.0);\n\
    }";

static fragment_src: &'static str =
   "#version 150\n\
    out vec4 outColor;\n\
    void main() {\n\
       outColor = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

fn main() {
    // Initialize GLFW
    let (mut glfw, mut window, events) = init_glfw();

    // Load shaders
    let shader_program = make_shader(vertex_src, fragment_src);

    // Load assets
    let triangle_buffer = make_triangle();
    let vertex_buffers = [
        triangle_buffer,
    ];

    // Main Loop
    while !window.should_close() {       
        glfw.poll_events();
        
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        render(window.render_context(), &vertex_buffers, shader_program);
    }
}

fn init_glfw() -> (glfw::Glfw, glfw::Window, Receiver<(f64, glfw::WindowEvent)>){
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw::WindowHint::ContextVersionMajor(3);
    glfw::WindowHint::ContextVersionMinor(2);
    glfw::WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core);
    glfw::WindowHint::OpenglForwardCompat(true);

    let (mut window, events) = glfw.create_window(
            640, 480, "GLFW Test Window",
            glfw::WindowMode::Windowed
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    // loading a specific function pointer
    gl::ClearColor::load_with(|s| window.get_proc_address(s));
    gl::Clear::load_with(|s| window.get_proc_address(s));
    gl::GenBuffers::load_with(|s| window.get_proc_address(s));
    gl::GenVertexArrays::load_with(|s| window.get_proc_address(s));
    gl::BindBuffer::load_with(|s| window.get_proc_address(s));
    gl::BindVertexArray::load_with(|s| window.get_proc_address(s));
    gl::EnableVertexAttribArray::load_with(|s| window.get_proc_address(s));
    gl::DisableVertexAttribArray::load_with(|s| window.get_proc_address(s));
    gl::VertexAttribPointer::load_with(|s| window.get_proc_address(s));
    gl::BufferData::load_with(|s| window.get_proc_address(s));
    gl::DrawArrays::load_with(|s| window.get_proc_address(s));
    gl::CreateShader::load_with(|s| window.get_proc_address(s));
    gl::ShaderSource::load_with(|s| window.get_proc_address(s));
    gl::CompileShader::load_with(|s| window.get_proc_address(s));
    gl::CreateProgram::load_with(|s| window.get_proc_address(s));
    gl::AttachShader::load_with(|s| window.get_proc_address(s));
    gl::BindFragDataLocation::load_with(|s| window.get_proc_address(s));
    gl::LinkProgram::load_with(|s| window.get_proc_address(s));
    gl::UseProgram::load_with(|s| window.get_proc_address(s));
    gl::GetAttribLocation::load_with(|s| window.get_proc_address(s));

    // Set the window clear color
    unsafe { gl::ClearColor(1.0, 0.0, 0.0, 1.0); }

    return (glfw, window, events)
}

fn render(mut context: glfw::RenderContext, buffers: &[GLuint], shader_program: GLuint) {
    context.make_current();
    
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        for buffer in buffers.iter() {
            let pos_attrib = gl::GetAttribLocation(
                shader_program,
                ffi::CString::from_slice("position".as_bytes()).as_ptr()
            ) as u32;
            gl::EnableVertexAttribArray(pos_attrib);
            gl::BindBuffer(gl::ARRAY_BUFFER, *buffer);
            gl::VertexAttribPointer(
                pos_attrib,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                0 as *const GLvoid
            );
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DisableVertexAttribArray(pos_attrib);
        }
    }
    
    context.swap_buffers();

    // Required on some platforms
    glfw::make_context_current(None);
}

fn make_shader(v_src :&str, f_src :&str) -> GLuint {
    let mut shader_program : GLuint = 0;
    unsafe {
        let vsrc = ffi::CString::from_slice(v_src.as_bytes());
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vsrc.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let fsrc = ffi::CString::from_slice(f_src.as_bytes());
        let fragment_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(fragment_shader, 1, &fsrc.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::BindFragDataLocation(shader_program, 0, ffi::CString::from_slice("outColor".as_bytes()).as_ptr());
        gl::LinkProgram(shader_program);
        gl::UseProgram(shader_program);
    }

    return shader_program;
}

fn make_triangle() -> GLuint {
    let mut vertex_buffer :GLuint = 0;
    let mut vertex_array :GLuint = 0;

    let vertex_data: [GLfloat; 9] = [
        -1.0, -1.0, 0.0,
        1.0, -1.0, 0.0,
        0.0, 1.0, 0.0,
    ];

    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array);
        gl::BindVertexArray(vertex_array);

        gl::GenBuffers(1, &mut vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

        let vertex_data_ptr: *const GLvoid = mem::transmute(&vertex_data[0]);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertex_data_ptr,
            gl::STATIC_DRAW
        );
    }

    return vertex_buffer;
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}