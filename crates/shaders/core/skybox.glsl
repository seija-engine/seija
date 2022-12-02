struct VSOutput {
  vec3 uv;
};

VSOutput vs_main() {
  VSOutput o;
  vec4 pos = vec4(vert_position, 1);
  mat4 proj = getCameraProj();
  mat4 view = getCameraView();
  vec4 outPos = proj * (mat4x4(mat3x3(view))  * pos);
 
  gl_Position = outPos.xyww;
  o.uv = vec3(vert_position.x + 1 / 2,vert_position.y + 1 / 2,vert_position.z + 1 / 2);
  return o;
}

vec4 fs_main(VSOutput ino) {
    vec3 uv = ino.uv;
    vec4 outColor = texture(samplerCube(tex_mainTexture,tex_mainTextureSampler), uv);
    outColor.rgb *= material.color.rgb;
    return outColor;
}

vec2 sampleSphericalMap(vec3 v)
{
    vec2 uv = vec2(atan(v.z, v.x), asin(-v.y));
    uv *= vec2(0.1591, 0.3183);
    uv += 0.5;
    return uv;
}

vec4 sphere_fs_main(VSOutput ino) {
    vec3 uv = ino.uv;
    vec2 uv2 = sampleSphericalMap(normalize(uv));
    vec4 color = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),uv2);
    return color;
}
