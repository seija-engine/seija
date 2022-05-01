#version 450

layout(location = 0) out vec4 o_Target;
layout(location = 3) in vec4 in_Color;
layout(location = 4) in vec2 in_Uv;

layout(set = 3, binding = 0) uniform texture2D mainTexture;
layout(set = 3, binding = 1) uniform sampler mainSampler;

void main() {
    vec4 outColor = texture(sampler2D(mainTexture,mainSampler),in_Uv);
    o_Target = outColor;
}