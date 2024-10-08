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
use std::sync::{Arc, Mutex};

use std::time::Instant;

fn create_mvp_matrix_from_cursor(cursor_position: (f64, f64), window_size: (f64, f64)) -> [[f32; 4]; 4] {
    // Calculate the center of the window
    let (center_x, center_y) = (window_size.0 / 2.0, window_size.1 / 2.0);

    // Calculate the offset of the cursor relative to the center
    let offset_x = (cursor_position.0 - center_x) / center_x;
    let offset_y = (center_y - cursor_position.1) / center_y; // Inverted Y axis to match screen coordinates

    // Scale the offsets for rotation (adjust sensitivity as needed)
    let rotation_x = offset_y as f32 * std::f32::consts::PI; // Pitch (rotate around X-axis)
    let rotation_y = offset_x as f32 * std::f32::consts::PI; // Yaw (rotate around Y-axis)

    // 1. Projection matrix (perspective)
    let aspect_ratio = window_size.0 as f32 / window_size.1 as f32;
    let fov: f32 = std::f32::consts::FRAC_PI_4; // 45 degrees field of view
    let near = 0.1;
    let far = 100.0;
    let projection = Perspective3::new(aspect_ratio, fov, near, far);

    // 2. View matrix (camera position and orientation)
    let eye = Point3::new(0.0, 0.0, 2.0); // Camera position
    let target = Point3::origin(); // Looking at the origin
    let up = Vector3::y(); // "Up" direction is the Y axis
    let view = Matrix4::look_at_rh(&eye, &target, &up);

    // 3. Model matrix (rotate based on cursor position)
    let rotation = Matrix4::from_euler_angles(rotation_x, rotation_y, 0.0);

    // Combine into the MVP matrix (Model * View * Projection)
    let mvp = projection.to_homogeneous() * view * rotation;

    // Convert nalgebra's Matrix4 to the [[f32; 4]; 4] format required by glium
    mvp.into()
}

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
    .with_title("Oxydian") .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let window_size = (1024.0, 768.0);
    // Track the start time
    let start_time = Instant::now();

    let shape = graphics::shapes::create_cube_3d();

    // Create the vertex buffer
    let vertex_buffer = glium::VertexBuffer::new(&display, shape.as_slice()).unwrap();

    // Create an empty index buffer (we’re not using indices here)
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Create the shader program
    let program = glium::Program::from_source(
        &display, graphics::shaders::VERTEX_SHADER_3D, graphics::shaders::FRAGMENT_SHADER, None).unwrap();
    let cursor_position = Arc::new(Mutex::new((0.0, 0.0)));
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
                    // Update the cursor position in the shared state
                    let mut cursor_pos = cursor_position.lock().unwrap();
                    *cursor_pos = (position.x, position.y);
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
                // This is where rendering would go
                // println!("Redraw requested!");
                // Start rendering to the frame
                let mut target = display.draw();

                // Clear the screen to black
                target.clear_color(0.0, 0.0, 0.0, 1.0);

                // Create the MVP matrix
                let cursor_pos = cursor_position.lock().unwrap();
                let mvp_matrix = create_mvp_matrix_from_cursor(*cursor_pos, window_size);

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
            }
            _ => (),
        }


    });
}