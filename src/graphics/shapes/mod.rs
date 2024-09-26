pub mod vertex;

use vertex::Vertex2D;

use vertex::Vertex3D;

pub fn create_rectangle_3d() -> Vec<Vertex3D> {
    vec![
        Vertex3D { position: [-0.5, -0.5, 0.0] },  // Bottom-left
        Vertex3D { position: [ 0.5, -0.5, 0.0] },  // Bottom-right
        Vertex3D { position: [-0.5,  0.5, 0.0] },  // Top-left
        Vertex3D { position: [-0.5,  0.5, 0.0] },  // Top-left (again)
        Vertex3D { position: [ 0.5, -0.5, 0.0] },  // Bottom-right (again)
        Vertex3D { position: [ 0.5,  0.5, 0.0] },  // Top-right
    ]
}
 
pub fn create_rectangle_2d() -> Vec<Vertex2D> {
    vec![
        Vertex2D { position: [-0.5, -0.5] },  // Bottom-left
        Vertex2D { position: [ 0.5, -0.5] },  // Bottom-right
        Vertex2D { position: [-0.5,  0.5] },  // Top-left
        Vertex2D { position: [-0.5,  0.5] },  // Top-left (again)
        Vertex2D { position: [ 0.5, -0.5] },  // Bottom-right (again)
        Vertex2D { position: [ 0.5,  0.5] },  // Top-right
    ]
}