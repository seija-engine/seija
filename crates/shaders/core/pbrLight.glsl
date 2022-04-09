use core.commonLight;

struct PixelParams {
    vec3  diffuseColor;
    vec3  f0;
    float roughness;
}

struct Light {
    int   typ;
    vec4  colorIntensity;
    vec3  l;
    float attenuation;
    float noL;
    vec3  worldPosition;
    vec3  direction;
};

struct MaterialInputs {
    vec4 baseColor;
    float roughness;
    float metallic;
    vec3  normal;

    vec3  specularColor;
    float glossiness;
}

void initMaterial(out MaterialInputs material) {
    material.baseColor = vec4(1.0);
    material.roughness = 1.0;
    material.metallic = 0.0;
    material.normal = vec3(0.0, 0.0, 1.0);
    material.glossiness = 0.0;
    material.specularColor = vec3(0.0);
}

float computePreExposedIntensity(const highp float intensity, const highp float exposure) {
    return intensity * exposure;
}

float getSquareFalloffAttenuation(float distanceSquare, float falloff) {
    float factor = distanceSquare * falloff;
    float smoothFactor = clamp(1.0 - factor * factor,0.0,1.0);
    // We would normally divide by the square distance here
    // but we do it at the call site
    return smoothFactor * smoothFactor;
}

float getDistanceAttenuation(const highp vec3 posToLight, float falloff) {
    float distanceSquare = dot(posToLight, posToLight);
    float attenuation = getSquareFalloffAttenuation(distanceSquare, falloff);
    // Assume a punctual light occupies a volume of 1cm to avoid a division by 0
    return attenuation * 1.0 / max(distanceSquare, 1e-4);
}

float getAngleAttenuation(const highp vec3 lightDir, const highp vec3 l, const highp vec2 scaleOffset) {
    float cd = dot(lightDir, l);
    float attenuation = clamp(cd * scaleOffset.x + scaleOffset.y,0.0,1,0);
    return attenuation * attenuation;
}


Light getLight(const uint index,vec3 vertPos) {
    Light light;
    light.typ                =  getLightsType(index);
    light.worldPosition      =  getLightsPosition(index);
    light.direction          =  getLightsDirection(index);
    light.colorIntensity.rgb = getLightsColor(index);
    float intensity          = getLightsIntensity(index);
    light.colorIntensity.w  = computePreExposedIntensity(intensity, getCameraExposure());
    light.noL                = clamp(dot(normal,light.l), 0.0, 1.0);
    vec3 posToLight = light.worldPosition - vertPos;
    float falloff   = getLightsFalloff(index);
    if(light.typ == eLIGHT_TYPE_DIR) {
        light.l = normalize(getLightsDirection(index));
        light.attenuation = 1;
    } else if (light.typ == eLIGHT_TYPE_POINT) {
        light.attenuation = getDistanceAttenuation(posToLight, falloff);
        light.l =  normalize(posToLight);
    } else if (light.typ == eLIGHT_TYPE_SPOT) {
        light.l =  normalize(posToLight);
        float scale  = getLightsSpotScale(index);
        float offset = getLightsSpotOffset(index);
        light.attenuation = getDistanceAttenuation(posToLight, falloff) * getAngleAttenuation(-light.direction,vec2(scale,offset));
    }
    return light;
}

void getPixelParams(const MaterialInputs material, out PixelParams pixel) {
   vec4 baseColor = material.baseColor;
  
}

vec4 evaluateLights(const MaterialInputs material) {
   PixelParams pixel;
   getPixelParams(material, pixel);
   return vec4(1.0);
}

vec4 evaluateMaterial(const MaterialInputs material) {
    vec4 color = evaluateLights(material);
    return color;
}
