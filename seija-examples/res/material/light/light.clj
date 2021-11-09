{
    :name "light"
    :order "Opaque"
    :light true
    :props [
        {:name "light" :type "float4" :default [1,1,1,1]}
        {:name "mainTexture" :type "Texture"}
    ]
    :pass {
        :front-face "Ccw"
        :z-write true
        :z-test "<"
        :cull "Back"
        :vs "res/material/light/vert.spv"
        :fs "res/material/light/frag.spv"
    }
}