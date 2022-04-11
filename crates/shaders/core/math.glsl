float max3(const vec3 v) {
    return max(v.x, max(v.y, v.z));
}

float pow5(float x) {
    float x2 = x * x;
    return x2 * x2 * x;
}
