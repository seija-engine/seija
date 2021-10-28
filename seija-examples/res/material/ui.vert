#version 450

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};


layout(location = 0) in vec3 Vertex_Position;

void main() {
    gl_Position = ViewProj  * vec4(Vertex_Position, 1.0);
}