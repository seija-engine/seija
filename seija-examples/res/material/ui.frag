#version 450

layout(location = 0) out vec4 o_Target;
layout(location = 3) in vec4 in_Color;

void main() {
    o_Target = vec4(in_Color.x,in_Color.y,in_Color.z,1);
}