use core.pbrLight;

struct VSOutput {
  vec3 normal;
  vec4 outPos;
 
};

VSOutput vs_main() {
  VSOutput vsOutput;
  mat4 trans = getTransform();
  vec3 normal = transpose(inverse(mat3x3(trans)))  *  vert_normal;
  vsOutput.normal = normal; 
  vec4 pos = trans * vec4(vert_position, 1.0);
  vsOutput.outPos = pos;
  pos = getCameraProjView() * pos;
  gl_Position =  pos;
  return vsOutput;
}



vec4 fs_main(VSOutput ino) {
    vec4 cameraPos = getCameraPosition();
    vec3 viewDir = normalize(cameraPos.xyz - ino.outPos.xyz);
    
    MaterialInputs inputs;
    initMaterial(inputs);

    inputs.normal = normalize(ino.normal);
    inputs.baseColor  = material.color;
    inputs.metallic   = material.metallic;
    
    vec4 evalColor = evaluateMaterial(inputs,ino.outPos.xyz,viewDir);
    return evalColor;
}
