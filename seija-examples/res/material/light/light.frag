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

void main() {
    vec4 textureColor = texture(sampler2D(mainTexture,mainSampler),in_Uv);
    vec3 N = normalize(in_Normal);
    vec3 L = normalize(directional_dir.xyz);
    vec3 R = reflect(-L,N);
    vec3 V = normalize(in_cameraPos.xyz - in_Pos.xyz);
    
    vec4 diffuse = directional_color * max(0,dot(N,L));

    vec4 specular = vec4(1,1,1,1) * pow(max(0,dot(V,R)),32);

   
    vec4 lightColor = ambient + diffuse + specular;

    o_Target = textureColor * lightColor;
   
  
}