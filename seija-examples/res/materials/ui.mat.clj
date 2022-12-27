{
    :name "baseColor"
    :order "Transparent"
    :props [
        {:name "color"       :type "float4" :default [0.2,0.2,0,1]}
    ]
    :pass [
        { 
            :shader { 
                :name "core.ui" 
                
            } 
            
        }

       
        
    ]
}