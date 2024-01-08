struct VSOutput {
  vec2 uv;
  vec3 outPos;
};

VSOutput sprite_vs_main() {
  VSOutput o;
  mat4 trans = getTransform();
  vec3 pos = vec3(trans * vec4(vert_position, 1.0));
  mat4 pv = getCameraProjView();
  vec4 glPos = pv * vec4(pos,1);
  gl_Position = glPos;
  o.outPos = glPos.xyz;
  vec3 uv3 = material.uvBuffer[vert_index0];
  o.uv = vec2(uv3.x,uv3.y);
  return o;
}

vec4 sprite_fs_main(VSOutput inv) {
    vec4 textureColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),inv.uv);
    if(textureColor.a < 0.001) {
        discard;
    }
    textureColor = textureColor * material.color;
    return textureColor;
}