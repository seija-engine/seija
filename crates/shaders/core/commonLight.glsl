struct Light {
    int typ;
    vec4  colorIntensity;
    vec3  l;
    float noL;
    vec3  worldPosition;
    float attenuation;
};

Light getLight(const int index,vec3 normal,vec3 vPos) {
    Light light;
    light.typ                = getLightsType(index);
    light.worldPosition      = getLightsPosition(index);
    if(light.typ == 0)  {
        light.attenuation = 1.0;
        light.l = normalize(getLightsDirection(index));
    } else {
        light.l =  normalize(light.worldPosition - vPos);
    }
    light.noL                = clamp(dot(normal,light.l), 0.0, 1.0);
    light.colorIntensity.rgb = getLightsColor(index);
    light.colorIntensity.w   = getLightsIntensity(index);
    return light;
}

