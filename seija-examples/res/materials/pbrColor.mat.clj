{
    :name "pbrColor"
    :order "Opaque"
    :props [
        {:name "metallic"          :type "float" :default 1 }
        {:name "roughness"        :type "float" :default 1 }
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass {
       
        :shader {
            :name "core.pbr"
            :macros []
        }
    }
}