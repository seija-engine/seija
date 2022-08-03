use core.math;
vec4 post_bloom(vec2 uv,texture2D mainTexture,sampler texSampler) {
    float luminanceThreshold = 0.01;
    vec4 origin = texture(sampler2D(mainTexture,texSampler),uv);

    float lum = clamp(luminance(origin) - luminanceThreshold, 0.0, 1.0);
    return vec4(origin.xyz * lum,1) ;
}