use core.commonLight;
use core.math;
use core.brdf;

struct PixelParams {
    vec3  diffuseColor;
    vec3  f0;
    float perceptualRoughness;
    float roughness;
    vec3 energyCompensation;
};

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
    
    float metallic;
    vec3  specularColor;
    float glossiness;
    vec3  normal;
};

void initMaterial(out MaterialInputs material) {
    material.baseColor = vec4(1.0);
    material.metallic = 0.0;
    material.normal = vec3(0.0, 0.0, 1.0);
    material.glossiness = 0.0;
    material.specularColor = vec3(0.0);
}

float computePreExposedIntensity(const  float intensity, const  float exposure) {
    return intensity * exposure;
}

float getSquareFalloffAttenuation(float distanceSquare, float falloff) {
    float factor = distanceSquare * falloff;
    float smoothFactor = clamp(1.0 - factor * factor,0.0,1.0);
    // We would normally divide by the square distance here
    // but we do it at the call site
    return smoothFactor * smoothFactor;
}

float getDistanceAttenuation(const  vec3 posToLight, float falloff) {
    float distanceSquare = dot(posToLight, posToLight);
    float attenuation = getSquareFalloffAttenuation(distanceSquare, falloff);
    // Assume a punctual light occupies a volume of 1cm to avoid a division by 0
    return attenuation * 1.0 / max(distanceSquare, 1e-4);
}

float getAngleAttenuation(const  vec3 lightDir, const  vec3 l, const  vec2 scaleOffset) {
    float cd = dot(lightDir, l);
    float attenuation = clamp(cd * scaleOffset.x + scaleOffset.y,0.0,1.0);
    return attenuation * attenuation;
}


Light getLight(const int index,vec3 vertPos,vec3 normal) {
    Light light;
    light.typ                =  getLightsType(index);
    light.worldPosition      =  getLightsPosition(index);
    light.direction          =  getLightsDirection(index);
    light.colorIntensity.rgb = getLightsColor(index);
    float intensity          = getLightsIntensity(index);
    light.colorIntensity.w  = computePreExposedIntensity(intensity, getExposure());
    vec3 posToLight = light.worldPosition - vertPos;
    float falloff   = getLightsFalloff(index);
    if(light.typ == eLIGHT_TYPE_DIR) {
        light.l = normalize(getLightsDirection(index));
        light.attenuation = 1.0;
    } else if (light.typ == eLIGHT_TYPE_POINT) {
        light.attenuation = getDistanceAttenuation(posToLight, falloff);
        light.l =  normalize(posToLight);
    } else if (light.typ == eLIGHT_TYPE_SPOT) {
        light.l =  normalize(posToLight);
        float scale  = getLightsSpotScale(index);
        float offset = getLightsSpotOffset(index);
        light.attenuation = getDistanceAttenuation(posToLight, falloff) * getAngleAttenuation(-light.direction,light.l,vec2(scale,offset));
    }
    light.noL  = clamp(dot(normal,light.l), 0.0, 1.0);
    return light;
}



void getPixelParams(const MaterialInputs material, out PixelParams pixel) {
   vec4 baseColor = material.baseColor;
   pixel.diffuseColor = baseColor.rgb * (1.0 - material.metallic);
   pixel.f0 = material.specularColor;
   float perceptualRoughness = 1.0 - material.glossiness;
   pixel.perceptualRoughness = clamp(perceptualRoughness, 0.045, 1.0);
   pixel.roughness = pixel.perceptualRoughness * pixel.perceptualRoughness;
   pixel.energyCompensation = vec3(1.0);
}

vec3 specularLobe(const PixelParams pixel, const Light light, const vec3 h,float noV, float noL, float noH, float loH) {
    float d = distribution(pixel.roughness, noH, h);
    float v = visibility(pixel.roughness, noV, noL);
    vec3  f = fresnel(pixel.f0, loH);
    return (d * v) * f;
}

vec3 diffuseLobe(const PixelParams pixel,float noV,float noL,float loH) {
    float d = diffuse(pixel.roughness,noV,noL,loH);
    return pixel.diffuseColor * d;
}

vec3 surfaceShading(const PixelParams pixel,const Light light,float occlusion,vec3 viewDir,vec3 normal) {
    vec3 h = normalize(viewDir + light.l);
    float noV = max(dot(normal, viewDir),1e-4);
    float noL = clamp(light.noL,0.0,1.0);
    float noH = clamp(dot(normal, h),0.0,1.0);
    float loH = clamp(dot(light.l, h),0.0,1.0);

    vec3 fr = specularLobe(pixel, light, h, noV, noL, noH, loH);
    vec3 fd = diffuseLobe(pixel, noV, noL, loH);

    vec3 color = fr;//fd + fr * pixel.energyCompensation;

    return (color * light.colorIntensity.rgb) * (light.colorIntensity.w * light.attenuation * noL * occlusion);
}

vec4 evaluateLights(const MaterialInputs material,vec3 vertPos,vec3 viewDir) {
   PixelParams pixel;
   getPixelParams(material, pixel);
   vec3 color = vec3(0.0);
   for(int i = 0; i < getLightCount();i++) {
      Light light = getLight(i,vertPos,material.normal);
      if (light.noL <= 0.0 || light.attenuation <= 0.0) {
            continue;
      }
      float visibility = 1.0;
      color.rgb += surfaceShading(pixel, light, visibility,viewDir,material.normal);
   }
   return vec4(color,1.0);
}

vec4 evaluateMaterial(const MaterialInputs material,vec3 vertPos,vec3 viewDir) {
    vec4 color = evaluateLights(material,vertPos,viewDir);
    return color;
}
