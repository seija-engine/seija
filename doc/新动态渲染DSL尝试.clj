typeclass IRenderPlugin where
    on-start
    on-update


(def base-plugin
    {
        :on-start (fn []
            
            (core/add-transform-ubo :auto)
        )
        
        :on-update (fn []
        
        )
    }    
)