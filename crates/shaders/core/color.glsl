struct VSOutput {
  vec3 color;
};

void vs_main() {
  vec4 pos = getTransform() * vec4(vert_position, 1.0);
  gl_Position = getCameraProjView() * pos;
}

vec4 fs_main() {
    vec4 color = vec4(1,0,0,1);
    color.a = 0.5;
    return color;
}
