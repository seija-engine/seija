{
    :name "bloom"
    :order "Opaque"
    
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
        {:name "texture0" :type "Texture" :default "white"}
        {:name "texture1" :type "Texture" :default "white"}
    ]
    :pass [
        {
            :z-write false
            :tag "PostEffect"
            :shader { :name "core.bloom_prefilter"  }
        }

        {
            :z-write false
            :tag "PostEffect"
            :shader { :name "core.bloom_frag_hor"  }
        }

        {
            :z-write false
            :tag "PostEffect"
            :shader { :name "core.bloom_frag_ver"  }
        }
    
        {
            :z-write false
            :tag "PostEffect"
            :shader { :name "core.postEffectAdd2"  }
        }

    ]
}