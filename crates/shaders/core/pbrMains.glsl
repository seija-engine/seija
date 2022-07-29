use core.pbrLight;

struct VSOutput {
  vec3 normal;
  vec4 outPos;
 #ifdef HAS_SHADOW
  vec4 outLightPos;
 #endif
};

VSOutput vs_main() {
  VSOutput vsOutput;
  mat4 trans = getTransform();
  vec3 normal = transpose(inverse(mat3x3(trans)))  *  vert_normal;
  vsOutput.normal = normal; 
  vec4 pos = trans * vec4(vert_position, 1.0);
  vsOutput.outPos = pos;
#ifdef HAS_SHADOW
  vsOutput.outLightPos = getProjView() * pos; 
#endif
  pos = getCameraProjView() * pos;

  gl_Position =  pos;
  return vsOutput;
}


float shadowCalculation(vec4 fragPosLightSpace)
{
    //执行透视除法
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    //变换到[0,1]的范围
    projCoords = projCoords * 0.5 + 0.5;
    //取得最近点的深度(使用[0,1]范围下的fragPosLight当坐标)
    float closestDepth = texture_ShadowMap(vec2( projCoords.x,1 - projCoords.y)).r;
    //取得当前片段在光源视角下的深度
    float currentDepth = projCoords.z;
    //检查当前片段是否在阴影中
    float bias = 0.09;
    float shadow = currentDepth - bias > closestDepth  ? 1.0 : 0.0;
    return currentDepth - closestDepth;
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
    #ifdef HAS_SHADOW
      float shadow = shadowCalculation(ino.outLightPos);
    
      evalColor = vec4(shadow,shadow,shadow, 1);
    #endif
    return evalColor;
}
