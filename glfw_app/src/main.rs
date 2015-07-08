extern crate glfw;
extern crate gl;

use std::sync::mpsc::Receiver;
use std::mem;
use std::ptr;
use std::ffi::CString;
use glfw::{Action, Context, Key};
use gl::types::*;

/*************************/
// Global static variables
/*************************/

// Shader sources
static VERTEX_SRC: &'static str =
   "#version 130\n\
    in vec3 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 1.0);\n\
    }";

static FRAGMENT_SRC: &'static str =
   "#version 130\n\
    out vec4 outColor;\n\
    void main() {\n\
       outColor = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

/********************/
// Main Program Entry
/********************/

fn main() {
    // Initialize GLFW
    let (mut glfw, mut window, events) = init_glfw();

    // Load shaders
    let shader_program = make_shader(VERTEX_SRC, FRAGMENT_SRC);

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

    // Cleanup
    unsafe {
        gl::DeleteProgram(shader_program);
        gl::DeleteBuffers(1, &triangle_buffer);
    }
}

/******************/
// Initialize GLFW
/******************/

fn init_glfw() -> (glfw::Glfw, glfw::Window, Receiver<(f64, glfw::WindowEvent)>){
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw::WindowHint::ContextVersion(3, 2);
    glfw::WindowHint::OpenGlForwardCompat(true);
    glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);

    let (mut window, events) = glfw.create_window(
            640, 480, "GLFW Test Window",
            glfw::WindowMode::Windowed
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    // loading specific function pointers
    /*gl::ClearColor::load_with(|s| window.get_proc_address(s));
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
    gl::GetAttribLocation::load_with(|s| window.get_proc_address(s));*/

    // Set the window clear color
    unsafe { gl::ClearColor(0.5, 0.0, 0.0, 1.0); }

    return (glfw, window, events)
}

fn render(mut context: glfw::RenderContext, buffers: &[GLuint], shader_program: GLuint) {
    context.make_current();

    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::UseProgram(shader_program);
        gl::BindFragDataLocation(shader_program, 0, CString::new("outColor").unwrap().as_ptr());

        for buffer in buffers.iter() {
            let pos_attrib = gl::GetAttribLocation(
                shader_program,
                CString::new("position").unwrap().as_ptr()
            ) as u32;
            gl::EnableVertexAttribArray(pos_attrib as GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, *buffer);
            gl::VertexAttribPointer(
                pos_attrib as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null()
            );
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DisableVertexAttribArray(pos_attrib as GLuint);
        }
    }

    context.swap_buffers();

    // Required on some platforms
    glfw::make_context_current(None);
}

fn make_shader(v_src :&str, f_src :&str) -> GLuint {
    let shader_program;

    unsafe {
        shader_program = gl::CreateProgram();

        let vsrc = CString::new(v_src.as_bytes()).unwrap();
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vsrc.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let mut vshader_status = gl::FALSE as GLint;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut vshader_status);

        if vshader_status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(vertex_shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", std::str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
        }

        let fsrc = CString::new(f_src.as_bytes()).unwrap();
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &fsrc.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let mut fshader_status = gl::FALSE as GLint;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut fshader_status);

        if fshader_status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(fragment_shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", std::str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
        }

        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut link_status = gl::FALSE as GLint;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut link_status);

        // Fail on error
        if link_status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(shader_program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", std::str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
        }
    }

    shader_program
}

fn make_triangle() -> GLuint {
    let mut vertex_buffer :GLuint = 0;
    let mut vertex_array :GLuint = 0;

    let vertex_data: [GLfloat; 9] = [
        -1.0, -1.0, 0.0,
        1.0, -1.0, 0.0,
        0.0, 1.0, 0.0
    ];

    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array);
        gl::BindVertexArray(vertex_array);

        gl::GenBuffers(1, &mut vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&vertex_data[0]),
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
