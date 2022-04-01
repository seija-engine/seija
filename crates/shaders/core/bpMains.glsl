use core.blinnPhongLight;
use core.commonLight;

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
    vec3 v = normalize(ino.outCameraPos.xyz - ino.outPos.xyz);
    vec3 pixelColor = getAmbileColor().rgb;
    for(int i = 0; i < getLightCount();i++) {
      Light light = getLight(i,ino.normal,ino.outPos.xyz);
      pixelColor = pixelColor + evalLight(light,ino.normal,v);
    }
    return vec4(pixelColor,1.0);
}
