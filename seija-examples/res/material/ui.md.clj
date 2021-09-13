{
    :name "ui-color"
    :order "Transparent"
    :props [
        {:name "scale" :type "Float" :default 3.14141},
        {:name "width" :type "Int"},
    ]
    :pass {
        :z-write true
        :z-test "<"
        :cull "Back"
        :vs "ui.vert"
        :fs "ui.frag"
    }
}