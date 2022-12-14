{
    :name "baseColor"
    :order "Opaque"
    :props [
        {:name "color"       :type "float4" :default [0,1,0,1]}
    ]
    :pass [
        { 
            :shader { 
                :name "core.color" 
                
            } 
        }

       
        
    ]
}