struct VSInput {
  vec2 uv;
};

VSInput line_vs_main() {
  VSInput o;
  o.uv = vert_uv0; 
  vec4 pos = getTransform() * vec4(vert_position, 1.0);
  gl_Position = getCameraProjView() * pos;
  return o;
}



float mlerp( float a, float b, float x) {
	return a + x * (b - a);
}


vec4 line_fs_main(VSInput o) {
  
  float _width = material.width;

  float lowx = step(_width,o.uv.x);
  float lowy = step(_width,o.uv.y);
  float highx = step(_width,1.0 - o.uv.x);
  float highy = step(_width,1.0 - o.uv.y);
  float num = lowx * lowy * highx * highy;
   
  
  if(1 - num <= 0) {
	discard;
  }

  return material.lineColor;
}