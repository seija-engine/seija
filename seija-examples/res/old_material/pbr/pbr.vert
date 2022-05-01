#version 450

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 ViewMat;
    mat4 ProjMat;
    vec4 cameraPos;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 TransMat;
};

layout(set = 2, binding = 0) uniform Material {
    vec4 MatColor;
};



layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;
layout(location = 3) in vec3 Vertex_Normal;
layout(location = 4) in vec4 Vertex_Tangent;

layout(location = 0) flat out vec4 out_cameraPos;
layout(location = 1) out vec4 out_Pos;
layout(location = 2) out vec3 out_Normal;
layout(location = 3) out vec4 out_Color;
layout(location = 4) out vec2 out_Uv;
layout(location = 5) out vec4 out_Tangent;

void main() {
    vec4 pos = TransMat * vec4(Vertex_Position, 1.0);
    out_Pos =  pos;
    gl_Position = ViewProj * pos;
    mat3 Model3x3 = transpose(inverse(mat3x3(TransMat)));
    vec3 normal =  Model3x3 *  Vertex_Normal;
    out_Normal = normal.xyz;
    out_Color = MatColor;
    out_Uv = Vertex_Uv;
    out_cameraPos = cameraPos;
    out_Tangent = vec4(Model3x3 *  Vertex_Tangent.xyz,Vertex_Tangent.w);
}