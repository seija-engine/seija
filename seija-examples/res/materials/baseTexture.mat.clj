{
    :name "baseTexture"
    :order "Opaque"
    :props [
        {:name "color"       :type "float4" :default [1,1,1,1]}
        {:name "mainTexture" :type "Texture" :default "white"}
    ]
    :pass [
       
        { 
            :shader { :name "core.texture"  } 
        }

       
        
    ]
}