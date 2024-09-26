#[macro_use]
extern crate winit;
extern crate glium;

use glium::{glutin, Surface};
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::ControlFlow;

mod graphics {
    pub mod shapes;
    pub mod shaders;
}

extern crate nalgebra as na;

use na::{Matrix4, Perspective3, Point3, Vector3};

use std::time::Instant;

fn create_mvp_matrix(start_time: Instant) -> [[f32; 4]; 4] {
    // Calculate elapsed time
    let elapsed = start_time.elapsed().as_secs_f32();

    // 1. Projection matrix (perspective)
    let aspect_ratio = 1024.0 / 768.0;  // Adjust according to your window size
    let fov: f32 = std::f32::consts::FRAC_PI_4;  // 45 degrees field of view
    let near = 0.1;
    let far = 100.0;
    let projection = Perspective3::new(aspect_ratio, fov, near, far);

    // 2. View matrix (camera position and orientation)
    let eye = Point3::new(0.0, 0.0, 2.0);  // Camera position
    let target = Point3::origin();  // Looking at the origin
    let up = Vector3::y();  // "Up" direction is the Y axis
    let view = Matrix4::look_at_rh(&eye, &target, &up);

    // 3. Model matrix (object position and rotation)
    let rotation = Matrix4::from_euler_angles(0.0, elapsed, 0.0);  // Rotate over time

    // Combine into the MVP matrix (Model * View * Projection)
    let mvp = projection.to_homogeneous() * view * rotation;

    // Convert nalgebra's Matrix4 to the [[f32; 4]; 4] format required by glium
    mvp.into()
}
fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
    .with_title("Oxydian");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Track the start time
    let start_time = Instant::now();

    let shape = graphics::shapes::create_rectangle_3d();

    // Create the vertex buffer
    let vertex_buffer = glium::VertexBuffer::new(&display, shape.as_slice()).unwrap();

    // Create an empty index buffer (we’re not using indices here)
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Create the shader program
    let program = glium::Program::from_source(
        &display, graphics::shaders::VERTEX_SHADER_3D, graphics::shaders::FRAGMENT_SHADER, None).unwrap();

    // Event loop to keep the window open
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Wait for events

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit; // Exit the application
                }
                WindowEvent::Resized(size) => {
                    println!("Window resized to {:?}", size);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    println!("Key event: {:?}", input);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    println!("Cursor moved to {:?}", position);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // This is where game logic would go
                // println!("Main events cleared, ready for updates.");
                display.gl_window().window().request_redraw();
            }
            Event::RedrawRequested(_) => {

            }
            _ => (),
        }

        // This is where rendering would go
        // println!("Redraw requested!");
        // Start rendering to the frame
        let mut target = display.draw();

        // Clear the screen to black
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Create the MVP matrix
        let mvp_matrix = create_mvp_matrix(start_time);

        // Create the shader program (ensure the shaders include the new uniform)
        let program = glium::Program::from_source(&display, graphics::shaders::VERTEX_SHADER_3D, graphics::shaders::FRAGMENT_SHADER, None).unwrap();

        // When drawing, pass the MVP matrix to the shader
        let uniforms = glium::uniform! {
            mvp: mvp_matrix,  // Pass the MVP matrix to the vertex shader
        };

        // Draw the object with the MVP matrix applied
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
                        // Finish rendering and swap buffers
        target.finish().unwrap();
    });
}