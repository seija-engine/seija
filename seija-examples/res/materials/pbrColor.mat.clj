{
    :name "pbrColor"
    :order "Opaque"
    :props [
        {:name "metallic"          :type "float" :default 0.5 }
        {:name "roughness"        :type "float" :default 0.6 }
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass [
       
        { 
            :shader 
            { 
                :name "core.pbr"  
                :slot "
                    void slot_fs_material(inout MaterialInputs inputs,vec2 uv,inout vec4 normal) {
                        inputs.baseColor  = material.color;
                        inputs.metallic   = material.metallic;
                        inputs.roughness   = material.roughness;
                    }
                "
            
            }
        }

       
        
    ]
}