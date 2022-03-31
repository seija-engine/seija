use core.commonLight;

vec3 blinnPhongSpecular(vec3 n,vec3 l,vec3 v) {
    vec3 h = normalize(v + l);
    float spec = pow(max(0,dot(n, h)), 32);
    return vec3(1,1,1) * spec;
}

Light processLight(Light light,vec3 vPos) {
  if(light.typ == eLIGHT_TYPE_SPOT) 
  {
     
  }
  else if(light.typ == eLIGHT_TYPE_POINT) 
  {
      float x = distance(light.worldPosition,vPos) / light.ex1;
      light.attenuation = 1 / (x * x);
  }
  return light;
}

vec3 evalLight(Light light,vec3 normal,vec3 v) {
      if(light.noL > 0.0) {
        vec3 lightColor = light.colorIntensity.rgb;
        vec3 diffuse = lightColor * light.noL;
        vec3 specular = blinnPhongSpecular(normal,light.l,v);
        vec3 outColor = specular + diffuse;
        outColor = outColor * light.colorIntensity.w * light.attenuation;
        return outColor;
      }
      return vec3(0,0,0);
}