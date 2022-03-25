struct VSOutput {
  vec4 color;
};

VSOutput vs_main() {
  VSOutput o;
  o.color = material.color; 
  vec4 pos = getTransform() * vec4(vert_position, 1.0);
  gl_Position = getCameraProjView() * pos;
  slot_vert_process();
  return o;
}

vec4 fs_main(VSOutput ino) {
   vec4 outColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),vec2(0,0));
    return outColor;
}
