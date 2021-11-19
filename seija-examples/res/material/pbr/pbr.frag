#version 450

layout(location = 0) out vec4 o_Target;


layout(location = 0) in vec4 in_cameraPos;
layout(location = 1) in vec4 in_Pos;
layout(location = 2) in vec3 in_Normal;
layout(location = 3) in vec4 in_Color;
layout(location = 4) in vec2 in_Uv;

layout(set = 4, binding = 0) uniform Light {
    vec4 ambient;
    vec4 directional_dir;
    vec4 directional_color;
};

layout(set = 3, binding = 0) uniform texture2D baseColorTexture;
layout(set = 3, binding = 1) uniform sampler baseColorSampler;

layout(set = 3, binding = 2) uniform texture2D roughnessTexture;
layout(set = 3, binding = 3) uniform sampler roughnessSampler;

layout(set = 3, binding = 3) uniform texture2D normalTexture;
layout(set = 3, binding = 4) uniform sampler normalSampler;

#define saturate(x) clamp(x, 0.0, 1.0)
#define MEDIUMP_FLT_MAX    65504.0
#define saturateMediump(x) min(x, MEDIUMP_FLT_MAX)

const float PI = 3.141592653589793;
const float gamma = 2.2;

float D_GGX(float roughness, float NoH, const vec3 n, const vec3 h) {
    vec3 NxH = cross(n, h);
    float a = NoH * roughness;
    float k = roughness / (dot(NxH, NxH) + a * a);
    float d = k * k * (1.0 / PI);
    return saturateMediump(d);
}


vec3 F_Schlick(float VoH, vec3 f0) {
    float f = pow(1.0 - VoH, 5.0);
    return f + f0 * (1.0 - f);
}

float F_Schlick(float VoH, float f0, float f90) {
    return f0 + (f90 - f0) * pow(1.0 - VoH, 5.0);
}

float pow5(float x) {
    float x2 = x * x;
    return x2 * x2 * x;
}

vec3 F_Schlick(const vec3 f0, float f90, float VoH) {
    // not using mix to keep the vec3 and float versions identical
    return f0 + (f90 - f0) * pow5(1.0 - VoH);
}

vec3 fresnel(vec3 f0, float LoH) {
    float f90 = saturate(dot(f0, vec3(50.0 * 0.33)));
    return F_Schlick(f0, f90, LoH);
}

float Fr_DisneyDiffuse(float NdotV, float NdotL, float LdotH, float roughness)
{
	float E_bias        = 0.0 * (1.0 - roughness) + 0.5 * roughness;
	float E_factor      = 1.0 * (1.0 - roughness) + (1.0 / 1.51) * roughness;
	float fd90          = E_bias + 2.0 * LdotH * LdotH * roughness;
	vec3  f0            = vec3(1.0);
	float light_scatter = F_Schlick(f0, fd90, NdotL).r;
	float view_scatter  = F_Schlick(f0, fd90, NdotV).r;
	return light_scatter * view_scatter * E_factor;
}

float D_GGX_TR(vec3 N, vec3 H, float a)
{
    float a2     = a*a;
    float NdotH  = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float nom    = a2;
    float denom  = (NdotH2 * (a2 - 1.0) + 1.0);
    denom        = PI * denom * denom;

    return nom / denom;
}

float V_SmithGGXCorrelated(float NoV, float NoL, float roughness) {
    float a2 = roughness * roughness;
    float GGXV = NoL * sqrt(NoV * NoV * (1.0 - a2) + a2);
    float GGXL = NoV * sqrt(NoL * NoL * (1.0 - a2) + a2);
    return 0.5 / (GGXV + GGXL);
}


void main() {
    vec4  textureColor = texture(sampler2D(baseColorTexture,baseColorSampler),in_Uv);
    vec4  roughnessColor = texture(sampler2D(roughnessTexture,roughnessSampler),in_Uv);
    vec4  normalColor = texture(sampler2D(normalTexture,normalSampler),in_Uv);
    float roughness = roughnessColor.r;

    vec3 normal = normalColor.rgb;   

    vec3 N = normalize(normal) + in_Normal;
    vec3 L = normalize(directional_dir.xyz);
    vec3 V = normalize(in_cameraPos.xyz - in_Pos.xyz);
    vec3 H = normalize(V + L);
    
    float NoL = max(dot(N, L),0);
    float NoV = max(dot(N, V), 0);
    float NoH = saturate(dot(N, H));
    float VoH = dot(V, H);
    float LoH =  saturate(dot(L, H));
  

    float Specular_D = D_GGX(roughness,NoH,N,H);
    float Specular_Vis = V_SmithGGXCorrelated(NoV,NoL,roughness);
    if(Specular_Vis > 1) {
        Specular_Vis = 1;
    }

    vec3 Specular_F = fresnel(vec3(roughness),LoH);
    vec3 specular = Specular_F * Specular_D  * Specular_Vis;
    float diffuse = Fr_DisneyDiffuse(NoV, NoL, LoH,roughness);
    

    o_Target = vec4(vec3(normal),1); 
}