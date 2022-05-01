#version 450

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 View;
    mat4 Proj;
};


layout(set = 2, binding = 0) uniform Material {
    vec4 MatColor;
};

layout(location = 0) in vec3 Vertex_Position;

layout(location = 3) out vec4 out_Color;
layout(location = 4) out vec3 out_Uv;

void main() {
    vec4 pos = vec4(Vertex_Position, 1.0);
    gl_Position = Proj * (mat4x4(mat3x3(View))  * pos);
    out_Color = MatColor;
    out_Uv = vec3(Vertex_Position.x + 1 / 2,Vertex_Position.y + 1 / 2,Vertex_Position.z + 1 / 2);
}