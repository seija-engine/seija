#version 450

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 TransMat;
};

layout(set = 2, binding = 0) uniform Material {
    vec4 MatColor;
};

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;

layout(location = 3) out vec4 out_Color;
layout(location = 4) out vec3 out_Uv;

void main() {
    vec4 pos = TransMat * vec4(Vertex_Position, 1.0);
    gl_Position = ViewProj * pos;
    out_Color = MatColor;
    out_Uv = Vertex_Position;
}