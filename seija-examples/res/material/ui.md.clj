{
    :name "ui-color"
    :order "Transparent"
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
        {:name "mainTexture" :type "Texture"}
    ]
    :pass {
        :front-face "Ccw"
        :z-write true
        :z-test "<"
        :cull "Back"
        :vs "res/material/vert.spv"
        :fs "res/material/frag.spv"
    }
}