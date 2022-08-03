use core.math;



vec4 frag_prefilter(vec2 uv,texture2D mainTexture,sampler texSampler) {
    ivec2 texSize = textureSize(mainTexture,0);
    vec2 texelSize = 1.0 / texSize;

    vec4 color  = texture(sampler2D(mainTexture,texSampler),uv);
    float luminanceThreshold = 0.01;
    
    float val = clamp(luminance(color) - luminanceThreshold, 0.0, 1.0);
    return vec4(color.rgb * val,color.a);
}

vec4 frag_downsample(vec2 uv,texture2D mainTexture,sampler texSampler) {
    ivec2 texSize = textureSize(mainTexture,0);
    vec2 texelSize = 1.0 / texSize;

    vec4 d = texelSize.xyxy * vec4(-1.0, -1.0, 1.0, 1.0);
    vec3 s;
    // Box filter
    vec3 s1 =texture(sampler2D(mainTexture,texSampler), uv + d.xy).rgb;
    vec3 s2 =texture(sampler2D(mainTexture,texSampler), uv + d.zy).rgb;
    vec3 s3 =texture(sampler2D(mainTexture,texSampler), uv + d.xw).rgb;
    vec3 s4 =texture(sampler2D(mainTexture,texSampler), uv + d.zw).rgb;

    float s1w = 1.0 / (brightness(s1) + 1.0);
    float s2w = 1.0 / (brightness(s2) + 1.0);
    float s3w = 1.0 / (brightness(s3) + 1.0);
    float s4w = 1.0 / (brightness(s4) + 1.0);
    s = (s1 * s1w + s2 * s2w + s3 * s3w + s4 * s4w) / (s1w + s2w + s3w + s4w);

    return vec4(s,1.0);
}

vec4 frag_upsample(vec2 uv,texture2D mainTexture,sampler texSampler) {
    ivec2 texSize = textureSize(mainTexture,0);
    vec2 texelSize = 1.0 / texSize;

    vec4 d = texelSize.xyxy * vec4(-1.0, -1.0, 1.0, 1.0);
    vec3 s;
    // Box filter
    vec3 s1 =texture(sampler2D(mainTexture,texSampler), uv + d.xy).rgb;
    vec3 s2 =texture(sampler2D(mainTexture,texSampler), uv + d.zy).rgb;
    vec3 s3 =texture(sampler2D(mainTexture,texSampler), uv + d.xw).rgb;
    vec3 s4 =texture(sampler2D(mainTexture,texSampler), uv + d.zw).rgb;

    float s1w = 1.0 / (brightness(s1) + 1.0);
    float s2w = 1.0 / (brightness(s2) + 1.0);
    float s3w = 1.0 / (brightness(s3) + 1.0);
    float s4w = 1.0 / (brightness(s4) + 1.0);
    s = (s1 * s1w + s2 * s2w + s3 * s3w + s4 * s4w) / (s1w + s2w + s3w + s4w);

    return vec4(s,1.0);
}