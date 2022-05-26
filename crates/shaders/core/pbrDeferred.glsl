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

  vec3 newPos = texture(sampler2D(tex_positionTexture,tex_positionTextureSampler),o.uv).rgb;
  
  vec4 normalColor = texture(sampler2D(tex_normalTexture,tex_normalTextureSampler),o.uv);
  vec4 diffColor = texture(sampler2D(tex_diffTexture,tex_diffTextureSampler),o.uv);
  vec4 specColor = texture(sampler2D(tex_specTexture,tex_specTextureSampler),o.uv);

  inputs.normal =   normalize(normalColor.xyz);
  inputs.baseColor  = diffColor;
  inputs.metallic   = specColor.b;
  inputs.roughness  = specColor.g;
  inputs.metallic   = 0.4;
  inputs.roughness  = 0.6;
  vec4 cameraPos = getCameraPosition();

  vec3 viewDir = normalize(cameraPos.xyz - newPos);
  
  
  //inputs.normal = vec3(0,1,0);
  vec4 evalColor = evaluateMaterial(inputs,newPos,viewDir);
 
 
  return evalColor;
}