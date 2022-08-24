{
    :name "pureColor"
    :order "Opaque"
    
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass [
        {
            :tag "Foward"
            :shader { :name "core.color" :macros [] }
        }

        {
            :tag "ShadowCaster"
            :targets []
            :shader { :name "core.shadowDepth" }
            :targets [
                        {
                            :format "Bgra8UnormSrgb" 
                            :blend { :color ["src" "-" "src"] :alpha nil } 
                        }
                     ]
        }
    ]
}