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
        :targets [{:format "Bgra8UnormSrgb" :blend nil} {:format "Bgra8UnormSrgb" :blend nil} ]
        :shader {
            :name "core.pbrGBuffer"
            :macros []
        }
    }
}