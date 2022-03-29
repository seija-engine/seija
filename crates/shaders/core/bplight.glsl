struct VSOutput {
  vec3 normal;
  vec4 outColor;
};

VSOutput vs_main() {
  VSOutput o;
  o.normal = vert_normal; 
  o.outColor = vec4(1,0,0,1);
  vec4 pos = getTransform() * vec4(vert_position, 1.0);
  gl_Position = getCameraProjView() * pos;
  int light_type = getLightsType(0);
 
  return o;
}

vec4 fs_main(VSOutput ino) {
    vec4 outColor = ino.outColor;
    
    
    return outColor;
}
