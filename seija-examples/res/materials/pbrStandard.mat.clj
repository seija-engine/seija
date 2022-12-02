{
    :name "pbrStandard"
    :order "Opaque"
    :props [
        {:name "baseColor"            :type "texture2D" :default "white" }
        {:name "emissive"             :type "texture2D" :default "black" }
        {:name "normal"               :type "texture2D" :default "blue" }
        {:name "metallicTex"          :type "texture2D" :default "white" }
        {:name "roughnessTex"         :type "texture2D" :default "white" }
        {:name "aoTexture"            :type "texture2D" :default "white" }

        {:name "metallicFactor" :type "float"  :default 1 }
        {:name "roughnessFactor" :type "float" :default 1 }
        {:name "baseColorFactor" :type "float4" :default [1,1,1,1] }
        {:name "emissiveFactor" :type  "float3" :default [1,1,1] }
        {:name "alphaCutoff" :type "float" :default 1}
    ]
    :pass [
       
        { 
           
            :shader { 
                :features ["NormalMap"]
                :name "core.pbr"
                :slot "
                    void slot_fs_material(inout MaterialInputs inputs,vec2 uv,out vec4 normalColor) {
                        inputs.baseColor = texture(sampler2D(tex_baseColor, tex_baseColorSampler), uv); 
                        inputs.baseColor = inputs.baseColor * material.baseColorFactor;
                       
                        vec4 m = texture(sampler2D(tex_metallicTex, tex_metallicTexSampler), uv);
                        vec4 r = texture(sampler2D(tex_roughnessTex, tex_roughnessTexSampler), uv);
                        inputs.metallic  = m.r * material.metallicFactor;
                        inputs.roughness = r.g * material.roughnessFactor;

                        inputs.emissiveColor = vec3(0);
                        normalColor = texture(sampler2D(tex_normal, tex_normalSampler), uv);
                        inputs.occlusion = 1;
                    }
                 "   
            }
        }

       
        
    ]
}