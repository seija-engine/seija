{
    :name "ui-color"
    :order "Transparent"
    :props [
        {:name "color" :type "float4" :default [0,0.8,1,1]}
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