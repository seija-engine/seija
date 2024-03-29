{
    :name "DeferredLightPass"
    :order "Opaque"
    :props [
        {:name "positionTexture" :type "Texture" :default "white"}
        {:name "normalTexture"   :type "Texture" :default "white"}
        {:name "diffTexture" :type "Texture" :default "white"}
        {:name "specTexture" :type "Texture" :default "white"}
    ]
    :pass {
        :z-write false
        :shader {
            :name "core.pbrDeferred"
            :macros []
        }
    }
}