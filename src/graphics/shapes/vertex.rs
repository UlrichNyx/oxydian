extern crate glium;

// Define the Vertex struct with a position field
#[derive(Copy, Clone)]
pub struct Vertex2D {
    pub position: [f32; 2],  // 2D position: [x, y]
}

// Implement the vertex trait for the Vertex2D struct using glium's macro
glium::implement_vertex!(Vertex2D, position);

#[derive(Copy, Clone)]
pub struct Vertex3D {
    pub position: [f32; 3],  // 3D position: [x, y, z]
}

// Implement the vertex trait for the Vertex3D struct using glium's macro
glium::implement_vertex!(Vertex3D, position);