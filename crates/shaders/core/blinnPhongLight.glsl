use core.commonLight;

vec3 blinnPhongSpecular(vec3 n,vec3 l,vec3 v) {
    vec3 h = normalize(v + l);
    float spec = pow(max(0,dot(n, h)), 32);
    return vec3(1,1,1) * spec;
}

vec3 evalLight(Light light,vec3 normal,vec3 v) {
      if(light.noL > 0.0) {
        vec3 lightColor = light.colorIntensity.rgb * light.colorIntensity.w;
        vec3 diffuse = lightColor * light.noL;
        vec3 specular = blinnPhongSpecular(normal,light.l,v);
        return specular + diffuse;
      }
      return vec3(0,0,0);
}