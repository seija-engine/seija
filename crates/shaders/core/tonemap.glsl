use core.math;

struct VSInput {
  vec2 uv;
};

VSInput vs_main() {
  VSInput o;
  o.uv = vert_uv0; 
  vec4 pos = vec4(vert_position, 1.0);
  gl_Position =  pos;
  return o;
}

vec3 reinhardToneMapping(vec3 color, float adapted_lum) 
{
    float mIDDLE_GREY = 1;
    color *= mIDDLE_GREY / adapted_lum;
    return color / (1.0 + color);
}

vec3 ceToneMapping(vec3 color, float adapted_lum) 
{
    return 1 - exp(-adapted_lum * color);
}

vec3 acesToneMapping(vec3 color, float adapted_lum)
{
	const float a = 2.51;
	const float b = 0.03;
	const float c = 2.43;
	const float d = 0.59;
	const float e = 0.14;

	color *= adapted_lum;
	return (color * (a * color + b)) / (color * (c * color + d) + e);
}

vec4 fs_main(VSInput o) {
  vec4 texColor = texture_PostTexture(o.uv);
  vec3 ldrColor = acesToneMapping(texColor.rgb,0.7);

  return vec4(ldrColor,1);
}

