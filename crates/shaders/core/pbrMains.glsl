use core.pbrLight;

struct VSOutput {
  vec3 normal;
  vec4 outPos;
  vec4 outCameraPos;
};

VSOutput vs_main() {
  VSOutput vsOutput;
  mat4 trans = getTransform();
  vec3 normal = transpose(inverse(mat3x3(trans)))  *  vert_normal;
  vsOutput.normal = normal; 
  vsOutput.outCameraPos = getCameraPosition();
  vec4 pos = trans * vec4(vert_position, 1.0);
  vsOutput.outPos = pos;
  pos = getCameraProjView() * pos;
  gl_Position =  pos;
  return vsOutput;
}



vec4 fs_main(VSOutput ino) {
    vec3 viewDir = normalize(ino.outCameraPos.xyz - ino.outPos.xyz);
    
    MaterialInputs inputs;
    initMaterial(inputs);

    inputs.normal = ino.normal;
    inputs.baseColor = vec4(1.0);
    inputs.metallic = 0.7;
    inputs.glossiness = 0.7;
    inputs.specularColor = vec3(1.0);
    
    vec4 evalColor = evaluateMaterial(inputs,ino.outPos.xyz,viewDir);
    return evalColor;
}
