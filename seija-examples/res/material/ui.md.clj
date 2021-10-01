{
    :name "ui-color"
    :order "Transparent"
    :props [
        {:name "scale" :type "float" :default 3.14141},
        {:name "width" :type "int"},
        {:name "position" :type "float3" :default [0,0,0]},
        {:name "bool3" :type "bool3" :default [true,true,true]}
    ]
    :pass {
        :z-write true
        :z-test "<"
        :cull "Back"
        :vs "ui.vert"
        :fs "ui.frag"
    }
}