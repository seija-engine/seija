{
    :name "skybox"
    :order "Opaque"
    :props [
        {:name "mainTexture" :type "CubeMap" :default "white"}
        {:name "color"       :type "float4" :default [1,1,1,1]}
    ]
    :pass [
        { 
            
            :cull "Off"
            :z-test "<="
            :shader { 
                :name "core.skybox"
            } 
        }
    ]
}