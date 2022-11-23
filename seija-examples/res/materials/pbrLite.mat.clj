{
    :name "pbrColor"
    :order "Opaque"
    :props [
        {:name "mainTexture" :type "texture2D" :default "white"}
        {:name "metallic"          :type "float" :default 0 }
        {:name "roughness"        :type "float" :default 0.6 }
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass [
       
        { 
            :shader 
            { 
                :name "core.pbr"   :features ["Shadow"] 
                :slot "
                    void slot_fs_material(inout MaterialInputs inputs,vec2 uv,inout vec4 normal) {
                        vec4 texColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),uv);
                        inputs.baseColor  = texColor;
                        inputs.metallic   = material.metallic;
                        inputs.roughness   = material.roughness;
                        inputs.occlusion = 1;
                    }
                "
            
            }
        }

        {
            :tag "ShadowCaster"
            :shader { :name "core.shadowDepth" }
            :targets []
        }

       
        
    ]
}