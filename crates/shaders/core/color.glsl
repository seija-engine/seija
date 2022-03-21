struct VSOutput {
  vec3 color;
};

VSOutput vs_main() {
  VSOutput o;
  vec4 pos = getTransform() * vec4(vert_position, 1.0);
  gl_Position = getCameraProjView() * pos;
  return o;
}

vec4 fs_main(VSOutput vo) {
    vec4 color = vec4(1,0,0,1);
    color.a = 0.5;
    return color;
}
