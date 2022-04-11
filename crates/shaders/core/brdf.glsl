use core.math;

float d_GGX(float roughness, float noH, const vec3 h) {
    float oneMinusNoHSquared = 1.0 - noH * noH;
    float a = noH * roughness;
    float k = roughness / (oneMinusNoHSquared + a * a);
    float d = k * k * (1.0 / 3.14159265359);
    return d;
}


float v_SmithGGXCorrelated(float roughness, float noV, float noL) {
    float a2 = roughness * roughness;
    float lambdaV = noL * sqrt((noV - a2 * noV) * noV + a2);
    float lambdaL = noV * sqrt((noL - a2 * noL) * noL + a2);
    float v = 0.5 / (lambdaV + lambdaL);
    return v;
}

vec3 f_Schlick(const vec3 f0, float f90, float voH) {
    return f0 + (f90 - f0) * pow5(1.0 - voH);
}

float f_Schlick_float(float f0, float f90, float voH) {
    return f0 + (f90 - f0) * pow5(1.0 - voH);
}


float distribution(float roughness, float noH, const vec3 h) {
    return d_GGX(roughness, noH, h);
}

float visibility(float roughness, float noV, float noL) {
    return v_SmithGGXCorrelated(roughness, noV, noL);
}

vec3 fresnel(const vec3 f0, float loH) {
    float f90 = clamp(dot(f0, vec3(50.0 * 0.33)),0.0,1.0);
    return f_Schlick(f0, f90, loH);
}

float fd_Burley(float roughness, float noV, float noL, float loH) {
    // Burley 2012, "Physically-Based Shading at Disney"
    float f90 = 0.5 + 2.0 * roughness * loH * loH;
    float lightScatter = f_Schlick_float(1.0, f90, noL);
    float viewScatter  = f_Schlick_float(1.0, f90, noV);
    return lightScatter * viewScatter * (1.0 / 3.14159265359);
}

float diffuse(float roughness, float noV, float noL, float loH) {
    return fd_Burley(roughness, noV, noL, loH);
}
