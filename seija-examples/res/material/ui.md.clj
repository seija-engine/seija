{
    :name "ui-color"
    :order "Transparent"
    :props [
        {:name "scale" :type "float" :default 3.14141},
        {:name "width" :type "int" :default 100},
        {:name "position" :type "float3" :default [0,0,0]}
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