{
    :name "tonemap"
    :order "Opaque"
    :props [
        {:name "mainTexture" :type "Texture" :default "white"}
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