{
    :name "DeferredLightPass"
    :order "Opaque"
    :props [
        {:name "positionTex" :type "Texture" :default "white"}
        {:name "normalTex"   :type "Texture" :default "white"}
        {:name "mainTexture" :type "Texture" :default "blue"}
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass {

        :shader {
            :name "core.quadTexture"
            :macros []
        }
    }
}