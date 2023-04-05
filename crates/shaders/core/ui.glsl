struct VSInput {
  vec2 uv;
};

VSInput ui_vs_main() {
  VSInput o;
  o.uv = vert_uv0;
  mat4 trans = getTransform();
  vec3 pos = vec3(trans * vec4(vert_position, 1.0));
  mat4 pv = getCameraProjView();
  gl_Position = pv * vec4(pos,1);
  return o;
}

vec4 ui_fs_main(VSInput inv) {
    vec4 textureColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),inv.uv);
    
    return textureColor;
}

vec4 text_fs_main(VSInput inv) {
    vec4 textureColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),inv.uv);
    if(textureColor.r < 0.0001) {
      discard;
    }
    return vec4(textureColor.r,textureColor.r,textureColor.r,textureColor.r);
}
