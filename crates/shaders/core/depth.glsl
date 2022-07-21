void depth_vs_main() {
   vec4 pos = vec4(vert_position, 1.0);
   pos = getProjView() * pos;
   gl_Position =  pos;
}

void depth_fs_main() {
  
}
