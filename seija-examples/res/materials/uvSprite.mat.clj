{
    :name "UVSprite"
    :order "Transparent"
    :props [
        {:name "mainTexture" :type "texture2D" :default "white"}
        {:name "color"       :type "float4" :default [1,1,1,1]}
        {:name "frameUV"    :type "float4"}
    ]
    :pass [
        { 
            :z-test "always"
            :z-write false
            :shader { 
                :name "core.ui" 
                
            } 
            
        }

       
        
    ]
}