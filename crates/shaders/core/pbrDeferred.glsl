struct VSOutput {
  vec2 uv;
};

VSOutput deferred_vs_main() {
  VSOutput o;
  o.uv = vert_uv0;
  gl_Position = vec4(vert_position, 1.0);
  return o;
}

vec4 deferred_fs_main(VSOutput o) {
 
  return vec4(0.3,0.3,0.3,1);
}