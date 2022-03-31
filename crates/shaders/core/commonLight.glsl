const int eLIGHT_TYPE_DIR   = 0;
const int eLIGHT_TYPE_SPOT  = 1;
const int eLIGHT_TYPE_POINT = 2;


struct Light {
    int typ;
    vec4  colorIntensity;
    vec3  l;
    vec3  dir;
    float noL;
    vec3  worldPosition;
    float attenuation;
    float ex1;
    float ex2;
    float ex3;
};



Light getLight(const int index,vec3 normal,vec3 vPos) {
    Light light;
    light.typ                = getLightsType(index);
    light.worldPosition      = getLightsPosition(index);
    light.ex1 = getLightsEx1(index);
    light.ex2 = getLightsEx2(index);
    light.ex3 = getLightsEx3(index);
    light.dir = normalize(getLightsDirection(index));
    if(light.typ == eLIGHT_TYPE_DIR) {
        light.l = normalize(getLightsDirection(index));
        light.attenuation = 1;
    } else {
        light.l =  normalize(light.worldPosition - vPos);
        float x = distance(light.worldPosition,vPos) / light.ex1;
        light.attenuation = 1 / (x * x);
    }
    light.noL                = clamp(dot(normal,light.l), 0.0, 1.0);
    light.colorIntensity.rgb = getLightsColor(index);
    light.colorIntensity.w   = getLightsIntensity(index);
    return light;
}


float getDistanceAttenuation(float d,float r) {
    return 0;
}