pub mod vertex;

use vertex::Vertex2D;

use vertex::Vertex3D;

pub fn create_cube_3d() -> Vec<Vertex3D> {
    vec![
        // Front face
        Vertex3D { position: [-0.5, -0.5, 0.5] },
        Vertex3D { position: [0.5, -0.5, 0.5] },
        Vertex3D { position: [-0.5, 0.5, 0.5] },
        Vertex3D { position: [0.5, -0.5, 0.5] },
        Vertex3D { position: [0.5, 0.5, 0.5] },
        Vertex3D { position: [-0.5, 0.5, 0.5] },

        // Back face
        Vertex3D { position: [-0.5, -0.5, -0.5] },
        Vertex3D { position: [-0.5, 0.5, -0.5] },
        Vertex3D { position: [0.5, -0.5, -0.5] },
        Vertex3D { position: [0.5, -0.5, -0.5] },
        Vertex3D { position: [-0.5, 0.5, -0.5] },
        Vertex3D { position: [0.5, 0.5, -0.5] },

        // Left face
        Vertex3D { position: [-0.5, -0.5, -0.5] },
        Vertex3D { position: [-0.5, -0.5, 0.5] },
        Vertex3D { position: [-0.5, 0.5, -0.5] },
        Vertex3D { position: [-0.5, -0.5, 0.5] },
        Vertex3D { position: [-0.5, 0.5, 0.5] },
        Vertex3D { position: [-0.5, 0.5, -0.5] },

        // Right face
        Vertex3D { position: [0.5, -0.5, -0.5] },
        Vertex3D { position: [0.5, 0.5, -0.5] },
        Vertex3D { position: [0.5, -0.5, 0.5] },
        Vertex3D { position: [0.5, -0.5, 0.5] },
        Vertex3D { position: [0.5, 0.5, -0.5] },
        Vertex3D { position: [0.5, 0.5, 0.5] },

        // Top face
        Vertex3D { position: [-0.5, 0.5, -0.5] },
        Vertex3D { position: [-0.5, 0.5, 0.5] },
        Vertex3D { position: [0.5, 0.5, -0.5] },
        Vertex3D { position: [-0.5, 0.5, 0.5] },
        Vertex3D { position: [0.5, 0.5, 0.5] },
        Vertex3D { position: [0.5, 0.5, -0.5] },

        // Bottom face
        Vertex3D { position: [-0.5, -0.5, -0.5] },
        Vertex3D { position: [0.5, -0.5, -0.5] },
        Vertex3D { position: [-0.5, -0.5, 0.5] },
        Vertex3D { position: [-0.5, -0.5, 0.5] },
        Vertex3D { position: [0.5, -0.5, -0.5] },
        Vertex3D { position: [0.5, -0.5, 0.5] },
    ]
}

pub fn create_rectangle_3d() -> Vec<Vertex3D> {
    vec![
        Vertex3D { position: [-0.5, -0.5, 0.0] },  
        Vertex3D { position: [ 0.5, -0.5, 0.0] }, 
        Vertex3D { position: [-0.5,  0.5, 0.0] }, 
        Vertex3D { position: [-0.5,  0.5, 0.0] },  
        Vertex3D { position: [ 0.5, -0.5, 0.0] },  
        Vertex3D { position: [ 0.5,  0.5, 0.0] },  
        Vertex3D { position: [ 0.5, 0.5, 0.5] }, 
        Vertex3D { position: [ -0.5,  -0.5, 0.5] },  
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