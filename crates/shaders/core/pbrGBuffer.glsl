struct V2S {
  vec4 position;
  vec3 normal;
  vec4 tangent;
  vec2 uv;
};

V2S pbr_gbuffer_vs_main() {
  V2S v2s;
  mat4 trans = getTransform();

  vec4 worldPosition = trans * vec4(vert_position, 1.0);
  gl_Position = getCameraProjView() * worldPosition;

  mat3 transposeTrans = transpose(inverse(mat3x3(trans)));
  vec3 normal = transposeTrans * vert_normal;
  vec4 tangent = vec4(transposeTrans * vert_tangent.xyz,vert_tangent.w);

  v2s.position = worldPosition;
  v2s.normal = normal;
  v2s.tangent = tangent;
  v2s.uv = vert_uv0;
  return v2s;
}

struct GBufferTexs {
   vec4 rt0;
   vec4 rt1;
   vec4 rt2;
   vec4 rt3;
};

GBufferTexs pbr_gbuffer_fs_main(V2S v2s) {
    vec4 normalColor = texture(sampler2D(tex_normalTexture,tex_normalTextureSampler),v2s.uv);
    vec4 diffColor = texture(sampler2D(tex_baseColor,tex_baseColorSampler),v2s.uv);
    vec4 specColor = texture(sampler2D(tex_metallicRoughness,tex_metallicRoughnessSampler),v2s.uv);

    vec3 n = normalize(v2s.normal);
    vec3 t = normalize(v2s.tangent.xyz);
    vec3 b = cross(n, t) * v2s.tangent.w;
    mat3 tbn = mat3(t, b, n);
    vec3 normal = normalColor.rgb * 2.0 - 1.0;
    vec3 out_normal = normalize(tbn * normal);

    GBufferTexs texs;
    texs.rt0 = v2s.position;
    texs.rt1 = vec4(out_normal,0);
    texs.rt2 = diffColor;
    texs.rt3 = specColor;
    return texs;
}