struct VSOutput {
  vec3 color;
};

VSOutput vs_main() {
  VSOutput o;
  return o;
}

vec4 fs_main(VSOutput vo) {
    vec4 color = vec4(1,0,0,1);
    color.a = 0.5;
    return color;
}
