
void color_vs_main() {
  mat4 trans = getTransform();
  vec3 pos = vec3(trans * vec4(vert_position, 1.0));
  mat4 pv = getCameraProjView();
  gl_Position = pv * vec4(pos,1);
}

vec4 color_fs_main() {
    vec4 outColor = material.color;
    return outColor;
}


struct VSInput {
  vec2 uv;
};

VSInput texture_vs_main() {
  VSInput o;
  o.uv = vert_uv0; 
  vec4 pos = getTransform() * vec4(vert_position, 1.0);
  gl_Position = getCameraProjView() * pos;
  return o;
}

vec4 texture_fs_main(VSInput o) {
  vec4 texColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),o.uv);
  //texColor = texColor * material.color;
  //slot_fs_material(texColor);
  return texColor;
}

TexVSOutput texture_quadv_main() {
  TexVSOutput o;
  o.uv = vert_uv0;
  gl_Position = vec4(vert_position, 1.0);
  return o;
}

vec4 texture_quadf_main(TexVSOutput o) {
  vec4 texColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),o.uv);
 
  return texColor;
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
  mat4 skinMat = calcSkinMat();
  vec4 pos = getTransform() * skinMat * vec4(vert_position, 1.0);
 
 
  gl_Position = getCameraProjView() * pos;
  return o;
}


