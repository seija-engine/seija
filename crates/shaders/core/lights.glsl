use core.blinnPhongLight;
use core.commonLight;

struct VSOutput {
  vec3 normal;
  vec4 outPos;
  vec4 outCameraPos;
};

VSOutput vs_main() {
  VSOutput vsOutput;
  vsOutput.normal = vert_normal; 
  vsOutput.outCameraPos = getCameraPosition();
  vec4 pos = getTransform() * vec4(vert_position, 1.0);
  
  gl_Position = getCameraProjView() * pos;
  vsOutput.outPos = pos;
 
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
