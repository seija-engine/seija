struct VSInput {
  vec2 uv;
  uint index;
};

VSInput ui_vs_main() {
  VSInput o;
  o.uv = vert_uv0;
  o.index = uint(vert_uv1.x);
  mat4 trans = getTransform();
  vec3 pos = vec3(trans * vec4(vert_position, 1.0));
  mat4 pv = getCameraProjView();
  gl_Position = pv * vec4(pos,1);
  return o;
}

vec4 ui_fs_main(VSInput inv) {
    vec4 outColor = material.color;
    float uv_x = inv.uv.x;
    float uv_y = inv.uv.y;

    vec4 color = vec4(1);
    if(color.a < 0.0001) {
      discard;
    }
    return color;
}
