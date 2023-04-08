{
    :name "text"
    :order "Transparent"
    :props [
        {:name "mainTexture" :type "texture2D" :default "white"}
        {:name "color"       :type "float4" :default [0.2,0.2,0,1]}
    ]
    :pass [
        { 
            :z-test "always"
            :z-write false
            :shader { 
                :name "core.text" 
                
            } 
            
        }

       
        
    ]
}