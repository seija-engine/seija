{
    :name "pbrColor"
    :order "Opaque"
    :props [
        {:name "baseColor"         :type "float4" :default [1,1,1,1]}
        {:name "metallic"          :type "float"  }
        {:name "roughness"         :type "float"  }
        {:name "reflectance "      :type "float"  }
        {:name "emissive"          :type "float4" :default [0,0,0,1]}
        {:name "ambientOcclusion"  :type "float" }
    ]
    :pass {
       
        :shader {
            :name "core.pbr"
            :macros []
        }
    }
}