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

layout(set = 4, binding = 0) uniform Light {
    vec4 ambient;
    vec4 directional_dir;
    vec4 directional_color;
};

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;
layout(location = 3) in vec3 Vertex_Normal;

layout(location = 3) out vec4 out_Color;
layout(location = 4) out vec2 out_Uv;

void main() {
    vec4 pos = TransMat * vec4(Vertex_Position, 1.0);
    gl_Position = ViewProj * pos;
    vec4 normal = TransMat *  vec4(Vertex_Normal,1.0);
    
    out_Color = ambient + (directional_color * max(0,dot(normal  ,directional_dir)));
    out_Uv = Vertex_Uv;
}