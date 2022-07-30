void depth_vs_main() {
   vec4 pos = getProjView() * getTransform() * vec4(vert_position, 1.0);
  
   gl_Position =  pos;
}

void depth_fs_main() {
   gl_FragDepth = gl_FragCoord.z;
}
