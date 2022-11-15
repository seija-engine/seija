struct VSInput {
  vec2 uv;
};

VSInput vs_main() {
  VSInput o;
  o.uv = vert_uv0; 
  vec4 pos = vec4(vert_position, 1.0);
  gl_Position = getCameraProjView() * pos;
  return o;
}

vec4 fs_main(VSInput o) {
  vec4 texColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),o.uv);
  texColor.a = texColor.a + 0.3;
  return texColor;
}