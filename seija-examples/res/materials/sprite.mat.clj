{
    :name "sprite"
    :order "Transparent"
    :props [
        {:name "mainTexture" :type "texture2D" :default "white"}
        {:name "color"       :type "float4" :default [1,1,1,1] }
        {:name "uvBuffer"    :type "float3[4]" }
    ]
    :pass [
        { 
            :z-test "always"
            :shader { 
                :name "core.sprite" 
                
            } 
            
        }

       
        
    ]
}