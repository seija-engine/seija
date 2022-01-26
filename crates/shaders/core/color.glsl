use core.math;

struct VSOutput {
  vec3 color;
  vec3 normal;
  vec2 uv;
};

void slot_vs(VSOutput vs) {}

VSOutput vs_main() {
   VSOutput output2;
   int a = 112;
   if(a > 5) return output2;
   else if(a < 2) return output2;
   else      return output2;
   slot_vs(output2);
   while(true) return output2;
   for(int i = 0;i < 10; i++) return output2;
   return output2;
}


vec4 fs_main(VSOutput vo) {
   vec4 color = vec4(1);
   if(vo.uv.x > 0) {
      return vec4(1);
   }
   return color;
}
//---->
/*

struct VSOutput {
  vec3 color;
  vec3 normal;
  vec2 uv;
};
layout(location = 0) VSOutput _VSOutput;
void vs_main() {
  _VSOutput.uv = vertex_uv0;
  gl_Position = vertex_position;
   return;
}
*/