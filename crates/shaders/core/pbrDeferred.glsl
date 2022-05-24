struct VSOutput {
  vec2 uv;
};

VSOutput deferred_vs_main() {
  VSOutput o;
  o.uv = vert_uv0;
  gl_Position = vec4(vert_position, 1.0);
  return o;
}

vec4 deferred_fs_main(VSOutput o) {
  vec4 posColor = texture(sampler2D(tex_positionTex,tex_positionTexSampler),o.uv);
  vec4 normalColor = texture(sampler2D(tex_normalTex,tex_normalTexSampler),o.uv);
  return vec4(posColor.r * normalColor.r,posColor.g * normalColor.g,posColor.b * normalColor.b,1);
}