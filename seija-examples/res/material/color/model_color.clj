{
    :name "model-color"
    :order "Transparent"
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass {
        :front-face "Ccw"
        :z-write true
        :z-test "<"
        :cull "Back"
        :vs "res/material/color/vert.spv"
        :fs "res/material/color/frag.spv"
    }
}