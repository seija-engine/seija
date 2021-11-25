#version 450

layout(location = 0) out vec4 o_Target;


layout(location = 0) in vec4 in_cameraPos;
layout(location = 1) in vec4 in_Pos;
layout(location = 2) in vec3 in_Normal;
layout(location = 3) in vec4 in_Color;
layout(location = 4) in vec2 in_Uv;
layout(location = 5) in vec4 in_Tangent;

layout(set = 4, binding = 0) uniform Light {
    vec4 ambient;
    vec4 directional_dir;
    vec4 directional_color;
};

layout(set = 3, binding = 0) uniform texture2D baseColorTexture;
layout(set = 3, binding = 1) uniform sampler baseColorSampler;

layout(set = 3, binding = 2) uniform texture2D roughnessTexture;
layout(set = 3, binding = 3) uniform sampler roughnessSampler;

layout(set = 3, binding = 4) uniform texture2D normalTexture;
layout(set = 3, binding = 5) uniform sampler normalSampler;

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

float D_GGX2(float NoH, float roughness) {
    float a = NoH * roughness;
    float k = roughness / (1.0 - NoH * NoH + a * a);
    return k * k * (1.0 / PI);
}

float D_GGX3(float roughness, float NoH, const vec3 h) {
    float oneMinusNoHSquared = 1.0 - NoH * NoH;
    float a = NoH * roughness;
    float k = roughness / (oneMinusNoHSquared + a * a);
    float d = k * k * (1.0 / PI);
    return d;
}


vec3 F_Schlick(float VoH, vec3 f0) {
    float f = pow(1.0 - VoH, 5.0);
    return f + f0 * (1.0 - f);
}

float F_Schlick(float VoH, float f0, float f90) {
    return f0 + (f90 - f0) * pow(1.0 - VoH, 5.0);
}

float Fd_Burley(float NoV, float NoL, float LoH, float roughness) {
    float f90 = 0.5 + 2.0 * roughness * LoH * LoH;
    float lightScatter = F_Schlick(NoL, 1.0, f90);
    float viewScatter = F_Schlick(NoV, 1.0, f90);
    return lightScatter * viewScatter * (1.0 / PI);
}

float V_SmithGGXCorrelated(float NoV, float NoL, float roughness) {
    float a2 = roughness * roughness;
    float GGXV = NoL * sqrt(NoV * NoV * (1.0 - a2) + a2);
    float GGXL = NoV * sqrt(NoL * NoL * (1.0 - a2) + a2);
    return saturate(0.5 / (GGXV + GGXL));
}


void main() {
    vec4  textureColor = texture(sampler2D(baseColorTexture,baseColorSampler),in_Uv);
    vec4  roughnessColor = texture(sampler2D(roughnessTexture,roughnessSampler),in_Uv);
    vec4  normalColor = texture(sampler2D(normalTexture,normalSampler),in_Uv);
    float roughness = roughnessColor.r;

 

    vec3 N = normalize(in_Normal);
    vec3 T = normalize(in_Tangent.xyz);
    vec3 B = cross(N, T) * in_Tangent.w;
    mat3 TBN = mat3(T, B, N);

    vec3 normal = normalColor.rgb * 2.0 - 1.0;
    N = normalize(TBN * normal);
    N = normalize(in_Normal);
    
    vec3 L = normalize(directional_dir.xyz);
    vec3 V = normalize(in_cameraPos.xyz - in_Pos.xyz);
    vec3 H = normalize(V + L);
    
    float NoL = saturate(dot(N, L));
    float NoV = saturate(dot(N, V));
    float NoH = saturate(dot(N, H));
    float VoH = saturate(dot(V, H));
    float LoH = saturate(dot(L, H));
   

    float Specular_D = D_GGX3(roughness,NoH,H);
   
    float Specular_G = V_SmithGGXCorrelated(NoV,NoL,roughness);
    vec3 Specular_F = F_Schlick(VoH,vec3(0.2));
    vec3 specular = Specular_F  * Specular_G * Specular_D;
    
    float diffuse = Fd_Burley(NoV, NoL, LoH,roughness);

    vec3 outColor = textureColor.rgb;
    if(NoL > 0) {
        outColor = outColor + textureColor.rgb * specular;
    }
    
    
    o_Target = vec4(outColor,1); 
}