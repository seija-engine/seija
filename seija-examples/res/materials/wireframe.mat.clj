{
    :name "wireframe"
    :order "Opaque"
    :props [
        {:name "lineColor"       :type "float4" :default [0,1,0,1]}
        {:name "width"       :type "float" :default 0.02}
    ]
    :pass [
       
        
         
        { 
            :cull "Front"
            :shader { 
                :name "core.wireframe" 
            } 
        }

        { 
             :cull "Back"
            :shader { 
                :name "core.wireframe" 
            } 
        }
        
    ]
}