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
            :shader { :name "core.pbr"  } 
        }

        {
            :tag "ShadowCaster"
            :targets []
            :shader { :name "core.shadowDepth"  }
        }
        
    ]
}