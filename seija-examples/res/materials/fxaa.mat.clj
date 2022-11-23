{
    :name "fxaa"
    :order "Opaque"
    
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
        {:name "texture0" :type "texture2D" :default "white"}
    ]
    :pass [
        {
            :tag "PostEffect"
            :shader { :name "core.fxaa"  }
        }

    ]
}