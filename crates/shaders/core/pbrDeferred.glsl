use core.pbrLight;

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
  MaterialInputs inputs;
  initMaterial(inputs);

  vec4 posColor = texture(sampler2D(tex_positionTexture,tex_positionTextureSampler),o.uv);
  vec4 normalColor = texture(sampler2D(tex_normalTexture,tex_normalTextureSampler),o.uv);
  vec4 diffColor = texture(sampler2D(tex_diffTexture,tex_diffTextureSampler),o.uv);
  vec4 specColor = texture(sampler2D(tex_specTexture,tex_specTextureSampler),o.uv);

  inputs.normal = normalize(normalColor.xyz);
  inputs.baseColor  = diffColor;
  inputs.metallic   = specColor.b;
  inputs.glossiness = (1 - specColor.g);
  inputs.specularColor = diffColor.rgb;

  vec4 cameraPos = getCameraPosition();
  vec3 viewDir = normalize(cameraPos.xyz - posColor.xyz);

  vec4 evalColor = evaluateMaterial(inputs,posColor.xyz,viewDir);
  return evalColor;
}