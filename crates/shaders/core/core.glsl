struct VSOutput {
  vec3 color;
};

VSOutput vs_main() {
  VSOutput output;
  return output;
}

vec4 fs_main(VSOutput vo) {
    vec4 color = vec4(1);
    return color;
}
