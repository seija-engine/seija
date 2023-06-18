{
    :name "baseColor"
    :order "Transparent"
    :props [
        {:name "mainTexture" :type "texture2D" :default "white"}
        {:name "color"       :type "float4" :default [0.2,0.2,0,1]}
        {:name "clipRect"    :type "float4"}
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