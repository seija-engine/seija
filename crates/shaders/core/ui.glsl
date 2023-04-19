struct VSInput {
  vec2 uv;
  vec4 color;
};

VSInput ui_vs_main() {
  VSInput o;
  o.uv = vert_uv0;
  o.color = vert_color;
  mat4 trans = getTransform();
  vec3 pos = vec3(trans * vec4(vert_position, 1.0));
  mat4 pv = getCameraProjView();
  gl_Position = pv * vec4(pos,1);
  return o;
}

vec4 ui_fs_main(VSInput inv) {
    vec4 textureColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),inv.uv);
    if(textureColor.a < 0.001) {
        discard;
    }
    return textureColor;
}

vec4 text_fs_main(VSInput inv) {
    vec4 textureColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),inv.uv);
    return vec4(inv.color.r,inv.color.g,inv.color.b,textureColor.r);
}
