use core.fxaa;
use core.bloom;
use core.math;

struct TexVSOutput {
  vec2 uv;
};

struct VSUV5 {
  vec2 uv[5];
};

TexVSOutput vs_main() {
  TexVSOutput o;
  o.uv = vert_uv0;
  gl_Position = vec4(vert_position, 1.0);
  return o;
}


vec4 fxaa_fs_main(TexVSOutput o) {
  return fxaa_console(o.uv,tex_texture0,tex_texture0Sampler);
}

vec4 fs_bloom_prefilter(TexVSOutput o) {
  return frag_prefilter(o.uv,tex_texture0,tex_texture0Sampler);
}

VSUV5 vertBlurHorizontal() {
  VSUV5 o;
  vec2 rawUV = vert_uv0;
  float blurSize = 15;
  ivec2 texSize = textureSize(tex_texture0,0);
  vec2 texelSize = 1.0 / texSize;
  
  o.uv[0] = rawUV;
	o.uv[1] = rawUV + vec2(texelSize.x * 1.0, 0.0) * blurSize;
	o.uv[2] = rawUV - vec2(texelSize.x * 1.0, 0.0) * blurSize;
	o.uv[3] = rawUV + vec2(texelSize.x * 2.0, 0.0) * blurSize;
	o.uv[4] = rawUV - vec2(texelSize.x * 2.0, 0.0) * blurSize;
  gl_Position = vec4(vert_position, 1.0);
  return o;
}

VSUV5 vertBlurVertical() {
  VSUV5 o;
  vec2 rawUV = vert_uv0;
  float blurSize = 15;
  ivec2 texSize = textureSize(tex_texture0,0);
  vec2 texelSize = 1.0 / texSize;
  
  o.uv[0] = rawUV;
	o.uv[1] = rawUV + vec2(0.0,texelSize.y * 1.0) * blurSize;
	o.uv[2] = rawUV - vec2(0.0,texelSize.y * 1.0) * blurSize;
	o.uv[3] = rawUV + vec2(0.0,texelSize.y * 2.0) * blurSize;
	o.uv[4] = rawUV - vec2(0.0,texelSize.y * 2.0) * blurSize;
  gl_Position = vec4(vert_position, 1.0);
  return o;
}

vec4 fs_fragBlur(VSUV5 i) {
    float weight[3] = { 0.4026, 0.2442, 0.0545 };
		vec3 sum = texture(sampler2D(tex_texture0,tex_texture0Sampler), i.uv[0]).rgb * weight[0];
    
		for (int it = 1; it < 3; it++) {
			sum +=  texture(sampler2D(tex_texture0,tex_texture0Sampler), i.uv[it * 2 - 1]).rgb * weight[it];
			sum +=  texture(sampler2D(tex_texture0,tex_texture0Sampler), i.uv[it * 2]).rgb * weight[it];
		}
    
    return vec4(sum,1);
}


vec4 fs_add(TexVSOutput o) {
  vec4 color0 = texture(sampler2D(tex_texture0,tex_texture0Sampler), o.uv);
  vec4 color1 = texture(sampler2D(tex_texture1,tex_texture1Sampler), o.uv);
  return vec4(color0.rgb + color1.rgb,1);
}