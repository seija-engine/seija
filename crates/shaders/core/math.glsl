float max3(const vec3 v) {
    return max(v.x, max(v.y, v.z));
}

float pow5(float x) {
    float x2 = x * x;
    return x2 * x2 * x;
}

float luminance(vec4 color) {
	return  0.2125 * color.r + 0.7154 * color.g + 0.0721 * color.b;
}