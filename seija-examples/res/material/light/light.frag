#version 450

layout(location = 0) out vec4 o_Target;


layout(location = 0) in vec4 in_cameraPos;
layout(location = 1) in vec4 in_Pos;
layout(location = 2) in vec3 in_Normal;
layout(location = 3) in vec4 in_Color;
layout(location = 4) in vec2 in_Uv;

layout(set = 4, binding = 0) uniform Light {
    vec4 ambient;
    vec4 directional_dir;
    vec4 directional_color;
};

layout(set = 3, binding = 0) uniform texture2D mainTexture;
layout(set = 3, binding = 1) uniform sampler mainSampler;



vec3 phongSpecular(vec3 N,vec3 L,vec3 V) {
    vec3 R = reflect(-L,N);
    return vec3(1,1,1) * pow(max(0,dot(V,R)),32);
}

vec3 blinnPhongSpecular(vec3 N,vec3 L,vec3 V) {
    vec3 H = normalize(V + L);
    float spec = pow(max(0,dot(N, H)), 32);
    return vec3(1,1,1) * spec;
}

void main() {
    vec4 textureColor = texture(sampler2D(mainTexture,mainSampler),in_Uv);
    vec3 N = normalize(in_Normal);
    vec3 L = normalize(directional_dir.xyz);
    vec3 V = normalize(in_cameraPos.xyz - in_Pos.xyz);
    
    vec3 diffuse = directional_color.rgb * max(0,dot(N,L));

    vec3 specular = phongSpecular(N,L,V);

   
    vec3 lightColor = ambient.rgb + diffuse + specular;

    o_Target = textureColor * vec4(lightColor,1); 
}