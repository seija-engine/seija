use core.commonLight;

vec3 blinnPhongSpecular(vec3 n,vec3 l,vec3 v) {
    vec3 h = normalize(v + l);
    float spec = pow(max(0,dot(n, h)), 32);
    return vec3(1,1,1) * spec;
}


vec3 evalLight(Light light,vec3 normal,vec3 v) {
      if(light.noL < 0.0) { return vec3(0,0,0); }
      float attenuation = light.attenuation;
      if(light.typ == eLIGHT_TYPE_SPOT) 
      {
          float theta = dot(light.l, normalize(light.dir));
          if(theta <= light.ex3) 
          {   
              return vec3(0,0,0);
          }
          float epsilon = light.ex2 - light.ex3;
          float intensity = clamp((theta - light.ex3) / epsilon, 0.0, 1.0);
          attenuation = attenuation * intensity;
      }
      vec3 lightColor = light.colorIntensity.rgb;
      vec3 diffuse = lightColor * light.noL;
      vec3 specular = blinnPhongSpecular(normal,light.l,v);
      vec3 outColor = specular + diffuse;
      outColor = outColor * light.colorIntensity.w * attenuation;
      outColor = outColor * light.colorIntensity.rgb;
      return outColor;
}