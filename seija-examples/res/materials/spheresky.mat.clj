{
    :name "spheresky"
    :order "Opaque"
    :props [
        {:name "mainTexture" :type "texture2D" :default "white"}
        {:name "color"       :type "float4" :default [1,1,1,1]}
    ]
    :pass [
        { 
            
            :cull "Off"
            :z-test "<="
            :shader { 
                :name "core.spheresky"
            } 
        }
    ]
}