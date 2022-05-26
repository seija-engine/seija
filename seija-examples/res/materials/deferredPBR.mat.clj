{
    :name "DeferredPBR"
    :order "Opaque"
    :path  "Deferred"
    :props [
        {:name "baseColor"          :type "Texture" :default "white"}
        {:name "metallicRoughness"  :type "Texture" :default "white"}
        {:name "normalTexture"      :type "Texture" :default "blue"}
        {:name "metallicFactor"     :type "float" :default "0"}
        {:name "roughnessFactor"     :type "float" :default "0"}
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