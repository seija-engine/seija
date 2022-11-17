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

vec4 fs_main(VSInput o) {
  vec4 texColor = texture_PostTexture(o.uv);
  return vec4(grayColor(texColor.xyz),1);
}