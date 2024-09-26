// Vertex shader source code
pub const VERTEX_SHADER_2D: &str = r#"
    #version 140
    in vec2 position;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);  // 2D position, z is set to 0.0
    }
"#;

pub const VERTEX_SHADER_3D: &str = r#"
#version 140
in vec3 position;

uniform mat4 mvp;  // Model-View-Projection matrix

void main() {
    gl_Position = mvp * vec4(position, 1.0);  // Apply the transformation
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
    #version 140
    out vec4 color;
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);  // Red color
    }
"#;

