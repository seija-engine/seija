{
    :name "DeferredPBR.mat"
    :order "Opaque"
    :path  "Deferred"
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass {
        :shader {
            :name "core.color"
            :macros []
        }
    }
}