use core.fxaa;

struct TexVSOutput {
  vec2 uv;
};

TexVSOutput fxaa_vs_main() {
  TexVSOutput o;
  o.uv = vert_uv0;
  gl_Position = vec4(vert_position, 1.0);
  return o;
}


vec4 fxaa_fs_main(TexVSOutput o) {
  return fxaa_console(o.uv,tex_mainTexture,tex_mainTextureSampler);
}