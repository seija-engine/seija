{
    :name "pbrColor"
    :order "Opaque"
    :props [
        {:name "metallic"          :type "float" :default 1 }
        {:name "glossiness"        :type "float" :default 1 }
    ]
    :pass {
       
        :shader {
            :name "core.pbr"
            :macros []
        }
    }
}