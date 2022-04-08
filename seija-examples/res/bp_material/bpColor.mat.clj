{
    :name "bpColor"
    :order "Opaque"
    :props [
        {:name "basecolor" :type "float4" :default [1,1,1,1]}
    ]
    :pass {
        :cull "Off"
        :shader {
            :name "core.bpColor"
            :macros []
        }
    }
}