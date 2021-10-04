{
    :name "ui-color"
    :order "Transparent"
    :props [
        {:name "scale" :type "float" :default 3.14141},
        {:name "width" :type "int" :default 100},
        {:name "position" :type "float3" :default [0,0,0]}
    ]
    :pass {
        :z-write true
        :z-test "<"
        :cull "Back"
        :vs "ui.vert"
        :fs "ui.frag"
    }
}