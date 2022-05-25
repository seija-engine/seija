{
    :name "DeferredPBR"
    :order "Opaque"
    :path  "Deferred"
    :props [
        {:name "baseColor"          :type "Texture" :default "white"}
        {:name "metallicRoughness"  :type "Texture" :default "white"}
        {:name "normalTexture"      :type "Texture" :default "blue"}
    ]
    :pass {
        :targets [{:format "Rgba32Float" :blend nil} 
                  {:format "Rgba32Float" :blend nil} 
                  {:format "Rgba32Float" :blend nil} 
                  {:format "Rgba32Float" :blend nil} ]
        :shader {
            :name "core.pbrGBuffer"
            :macros []
        }
    }
}