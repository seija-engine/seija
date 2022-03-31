{
    :name "bplight"
    :order "Opaque"
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass {
        :cull "Off"
        :shader {
            :name "core.bplight"
            :macros []
        }
    }
}