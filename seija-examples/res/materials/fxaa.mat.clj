{
    :name "fxaa"
    :order "Opaque"
    
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass [
        {
            :tag "PostEffect"
            :shader { :name "core.color"  }
        }

    ]
}