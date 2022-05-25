{
    :name "DeferredLightPass"
    :order "Opaque"
    :props [
        {:name "positionTex" :type "Texture" :default "white"}
        {:name "normalTex"   :type "Texture" :default "white"}
        {:name "diffTexture" :type "Texture" :default "white"}
        {:name "specTexture" :type "Texture" :default "white"}
    ]
    :pass {

        :shader {
            :name "core.pbrDeferred"
            :macros []
        }
    }
}