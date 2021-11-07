#version 450
layout(location = 0) out vec4 o_Target;
layout(location = 3) in vec4 in_Color;
layout(location = 4) in vec3 in_Uv;

layout(set = 3, binding = 0) uniform textureCube cubeMap;
layout(set = 3, binding = 1) uniform sampler mainSampler;
void main() {
    vec4 outColor = texture(samplerCube(cubeMap,mainSampler), in_Uv);
    o_Target = outColor * in_Color;
}