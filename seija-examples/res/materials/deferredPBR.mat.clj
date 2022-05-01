{
    :name "DeferredPBR.mat"
    :order "Opaque"
    :path  "Deferred"
    :props [
        {:name "baseColor"          :type "Texture" :default "white"}
        {:name "metallicRoughness"  :type "Texture" :default "white"}
        {:name "normalTexture"      :type "Texture" :default "blue"}
    ]
    :pass {
        :shader {
            :name "core.pbrGBuffer"
            :macros []
        }
    }
}