use core.math;
float mmax3(vec3 v) {
    return max(v.x, max(v.y, v.z));
}

float pow5(float x) {
    float x2 = x * x;
    return x2 * x2 * x;
}

float luminance(vec4 color) {
	return  0.2125 * color.r + 0.7154 * color.g + 0.0721 * color.b;
}

vec3 median3(vec3 a, vec3 b, vec3 c)
{
    return a + b + c - max(max(a, b), c) - min(min(a, b), c);
}

float brightness(vec3 v) {
    return mmax3(v);
}


vec3 grayColor(vec3 color) {
    float grayValue = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;
    return vec3(grayValue,grayValue,grayValue);
}

float fd_Lambert() {
    return 1.0 / 3.14159265359;
}
