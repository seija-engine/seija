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
    float roughness;
    vec3  normal;
    vec3  emissiveColor;

    float occlusion;
};

void initMaterial(out MaterialInputs inputs) {
    inputs.baseColor = vec4(1.0);
    inputs.metallic = 0.0;
    inputs.normal = vec3(0.0, 0.0, 1.0);
    inputs.roughness = 0.0;
    inputs.emissiveColor = vec3(0.0);

    inputs.occlusion = 1.0;
}

vec3 computeF0(const vec4 baseColor, float metallic, float reflectance) {
    return baseColor.rgb * metallic + (reflectance * (1.0 - metallic));
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

float getDistanceAttenuation_old(const  vec3 posToLight, float falloff) {
    float distanceSquare = dot(posToLight, posToLight);
    float attenuation = getSquareFalloffAttenuation(distanceSquare, falloff);
    // Assume a punctual light occupies a volume of 1cm to avoid a division by 0
    return attenuation * 1.0 / max(distanceSquare, 1e-4);
}




float getDistanceAttenuation(vec3 posToLight,float falloff) {
    float rangeSqr = falloff * falloff;
    float distanceSquare = dot(posToLight, posToLight);
    float x2 = distanceSquare / rangeSqr;
    float x4 = x2 * x2;
    float oneMinuseX4 =  clamp(1 - x4, 0.0, 1.0);
    float smoothFactor = oneMinuseX4 * oneMinuseX4;
    return smoothFactor / x2;
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
    light.direction          =  -getLightsDirection(index);
    light.colorIntensity.rgb = getLightsColor(index);
    float intensity          = getLightsIntensity(index);
    light.colorIntensity.w  = computePreExposedIntensity(intensity, getExposure());
    vec3 posToLight = light.worldPosition - vertPos;
    float falloff   = getLightsFalloff(index);
    if(light.typ == eLIGHT_TYPE_DIR) {
        light.l = -normalize(getLightsDirection(index));
        light.attenuation = 1.0;
    } else if (light.typ == eLIGHT_TYPE_POINT) {
        light.attenuation = getDistanceAttenuation(posToLight,falloff);
        light.l = normalize(posToLight);
    } else if (light.typ == eLIGHT_TYPE_SPOT) {
        light.l =  normalize(posToLight);
        float scale  = getLightsSpotScale(index);
        float offset = getLightsSpotOffset(index);
        light.attenuation = getDistanceAttenuation(posToLight, falloff) * getAngleAttenuation(-light.direction,light.l,vec2(scale,offset));
    }
    light.noL  = clamp(dot(normal,light.l), 0.0, 1.0);
    return light;
}

void getPixelParams(const MaterialInputs inputs, out PixelParams pixel) {
   vec4 baseColor = inputs.baseColor;
   pixel.diffuseColor = baseColor.rgb * (1.0 - inputs.metallic);
   pixel.f0 = computeF0(baseColor,inputs.metallic,0.04);
   float perceptualRoughness = inputs.roughness;
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
   

    vec3 color = fd + fr * pixel.energyCompensation;
    
    return (color * light.colorIntensity.rgb) * (light.colorIntensity.w * light.attenuation * noL * occlusion);
}

vec4 evaluateLights(PixelParams pixel,const MaterialInputs inputs,vec3 vertPos,vec3 viewDir) {
   vec3 color = vec3(0.0);
   
   evaluateIBL(pixel,inputs,color,viewDir);

   for(int i = 0; i < getLightCount();i++) {
      Light light = getLight(i,vertPos,inputs.normal);
      if (light.noL <= 0.0 || light.attenuation <= 0.0) {
            continue;
      }
      float visibility = 1.0;
      color.rgb += surfaceShading(pixel, light, visibility,viewDir,inputs.normal);
   }
   
   return vec4(color,inputs.baseColor.a);
}

vec4 evaluateMaterial(MaterialInputs inputs,vec3 vertPos,vec3 viewDir) {
    PixelParams pixel;
    getPixelParams(inputs, pixel);
    
    vec4 color = evaluateLights(pixel,inputs,vertPos,viewDir);
   
    return color;
}

vec3 lerpV3(vec3 a,vec3 b,float v) {
    return vec3(mix(a.x,b.x,v),mix(a.y,b.y,v),mix(a.z,b.z,v));
}

void evaluateIBL(const PixelParams pixel,const MaterialInputs inputs,inout vec3 color,vec3 viewDir) {
    vec3 irradiance = texture(samplerCube(iblenv_irradianceMap,iblenv_irradianceMapS), inputs.normal).rgb;
    vec3 ambient = pixel.diffuseColor * irradiance * fd_Lambert();
    
    vec3 r = reflect(-viewDir,inputs.normal);
    vec2 ldfg = textureLod(sampler2D(iblenv_brdfLUT,iblenv_brdfLUTS), vec2(dot(inputs.normal, viewDir), pixel.perceptualRoughness), 0.0).xy;
    float lodRoughness = computeLODFromRoughness(pixel.perceptualRoughness);
    vec4 specularEnvironment = texture(samplerCube(iblenv_prefilterMap,iblenv_prefilterMapS), r, lodRoughness);
  
    vec3 dielectricColor  = vec3(0.0);
    float metallic = inputs.metallic;
    vec3 specular  = mix(dielectricColor, inputs.baseColor.rgb,metallic) * ldfg.x;

    specular  = specularEnvironment.rgb * specular;

    /*
    vec3 irradiance = texture(irradianceMap, N).rgb;
vec3 diffuse    = irradiance * albedo;

const float MAX_REFLECTION_LOD = 4.0;
vec3 prefilteredColor = textureLod(prefilterMap, R,  roughness * MAX_REFLECTION_LOD).rgb;   
vec2 envBRDF  = texture(brdfLUT, vec2(max(dot(N, V), 0.0), roughness)).rg;
vec3 specular = prefilteredColor * (F * envBRDF.x + envBRDF.y);

vec3 ambient = (kD * diffuse + specular) * ao; 
    */

    color.rgb += ambient;
    color.rgb += specular.rgb;
}

float computeLODFromRoughness(float perceptualRoughness) {
    float roughnessMipCount = 9; 
    return perceptualRoughness * roughnessMipCount;
}