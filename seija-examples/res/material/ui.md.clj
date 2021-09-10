{
    :name "ui-color"
    :order "Transparent"
    :props [
        {:name "texture"},
        {:name "color"  },
    ]
    :pass {
        :z-write true
        :z-test "<"
        :cull "Back"
        :vs "ui.vert"
        :fs "ui.frag"
    }
}