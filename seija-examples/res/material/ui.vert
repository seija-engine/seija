#version 450

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 TransMat;
};


layout(location = 0) in vec3 Vertex_Position;

void main() {
    vec4 pos = TransMat * vec4(Vertex_Position, 1.0);
    gl_Position = ViewProj * pos;
}