use core.pbrLight;

struct VSOutput {
  vec3 normal;
  vec3 outPos;
 #ifdef HAS_SHADOW
  vec4 outLightPos;
 #endif
};

VSOutput vs_main() {
  VSOutput vsOutput;
  mat4 trans = getTransform();
  vec3 normal = transpose(inverse(mat3x3(trans)))  *  vert_normal;
  vsOutput.normal = normal; 
  vec3 pos = vec3(trans * vec4(vert_position, 1.0));
  vsOutput.outPos = pos;
#ifdef HAS_SHADOW
  vsOutput.outLightPos =  getProjView() * vec4(vsOutput.outPos, 1.0); 
#endif
  

  gl_Position =  getCameraProjView() * vec4(pos, 1.0);
  return vsOutput;
}


float shadowCalculation(vec4 fragPosLightSpace)
{
    //执行透视除法
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    
    //变换到[0,1]的范围
    vec2 uvCoords = projCoords.xy * vec2(0.5,-0.5)  + vec2(0.5,0.5);
    //取得当前片段在光源视角下的深度
    float currentDepth = projCoords.z;
    //检查当前片段是否在阴影中
    float bias = getBias();
   
    ivec2 shadowSize = textureSize_ShadowMap();
    vec2 texelSize = 1.0 / shadowSize;
    float shadow = 0.0;
    for(int x = -1; x <= 1; ++x)
    {
      for(int y = -1; y <= 1; ++y)
      {
          float pcfDepth = texture_ShadowMap(uvCoords + vec2(x, y) * texelSize).r; 
          shadow += currentDepth - bias > pcfDepth ? 1.0 : 0.0;        
      }    
    }
    shadow /= 9.0;

    return shadow;
}

vec4 fs_main(VSOutput ino) {
    vec4 cameraPos = getCameraPosition();
    vec3 viewDir = normalize(cameraPos.xyz - ino.outPos);
    
    MaterialInputs inputs;
    initMaterial(inputs);

    inputs.normal = normalize(ino.normal);
    inputs.baseColor  = material.color;
    inputs.metallic   = material.metallic;
    
    vec4 evalColor = evaluateMaterial(inputs,ino.outPos,viewDir);
    #ifdef HAS_SHADOW
      float shadow = shadowCalculation(ino.outLightPos);
    
      evalColor = vec4(vec3( (1- shadow) * evalColor.xyz ), 1);
    #endif
    return evalColor;
}
