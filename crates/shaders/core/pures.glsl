struct VSOutput {
  vec4 color;
};

VSOutput color_vs_main() {
  VSOutput o;
  o.color = material.color; 
  vec4 pos = getTransform() * vec4(vert_position, 1.0);
  gl_Position = getCameraProjView() * pos;
  return o;
}

vec4 color_fs_main(VSOutput ino) {
    vec4 outColor = ino.color;
    return outColor;
}


struct TexVSOutput {
  vec2 uv;
  vec4 color;
};

TexVSOutput texture_vs_main() {
  TexVSOutput o;
  o.uv = vert_uv0; 
  o.color = material.color;
  vec4 pos = getTransform() * vec4(vert_position, 1.0);
  gl_Position = getCameraProjView() * pos;
  return o;
}

mat4 calcSkinMat() {
    mat4[256] jointMats = getJointMats();
    mat4 skinMat = vert_weights.x * jointMats[vert_joints.x] +
                   vert_weights.y * jointMats[vert_joints.y] +
                   vert_weights.z * jointMats[vert_joints.z] +
                   vert_weights.w * jointMats[vert_joints.w];
    return skinMat;
}

TexVSOutput texture_skin_vs_main() {
  TexVSOutput o;
  o.uv = vert_uv0; 
  o.color = material.color;
  mat4 skinMat = calcSkinMat();
  vec4 pos = getTransform() * skinMat * vec4(vert_position, 1.0);
 
 
  gl_Position = getCameraProjView() * pos;
  return o;
}


vec4 texture_fs_main(TexVSOutput o) {
  vec4 outColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),o.uv);
  outColor = outColor * o.color;
  return outColor;
}
