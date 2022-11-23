{
    :name "tonemap"
    :order "Opaque"
    :props [
        {:name "mainTexture" :type "texture2D" :default "white"}
        {:name "color"       :type "float4" :default [1,1,1,1]}
    ]
    :pass [
        { 
            :tag "PostEffect"
            :shader { 
                :name "core.tonemaping"
            } 
        }
    ]
}