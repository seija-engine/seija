{
    :name "baseColor"
    :order "Transparent"
    :props [
        {:name "mainTexture" :type "texture2D" :default "white"}
        {:name "color"       :type "float4" :default [1,1,1,1]}
        {:name "clipRect"    :type "float4"}
        {:name "isClip"      :type "int"}
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