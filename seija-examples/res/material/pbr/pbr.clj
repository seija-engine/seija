{
    :name "pbr"
    :order "Opaque"
    :light true
    
    
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
        {:name "baseColor" :type "Texture" :default "white"}
        {:name "roughness" :type "Texture"}
        {:name "normal" :type "Texture" :default "blue"}
    ]
    :pass {
        :front-face "Ccw"
        :z-write true
        :z-test "<"
        :cull "Back"
        :vs "res/material/pbr/vert.spv"
        :fs "res/material/pbr/frag.spv"
        :shader { 
            :macros ["NORMAL_MAP","FRESNEL"]
            :name "pbr"
            :frag "void on_material(MaterialInput input) {
                   
            }"    
        }
    }
}