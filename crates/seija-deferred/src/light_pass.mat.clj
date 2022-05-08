{
    :name "DeferredLightPass"
    :order "Opaque"
    :props [
        {:name "positionTex" :type "Texture" :default "white"}
        {:name "normalTex"   :type "Texture" :default "white"}
    ]
    :pass {

        :shader {
            :name "core.skinTexture"
            :macros []
        }
    }
}