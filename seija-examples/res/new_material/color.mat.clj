{
    :name "purecolor"
    :order "Opaque"
    :light true
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
        {:name "mainTexture" :type "Texture" :default "blue"}
    ]
    :pass {
        :shader {
            :name "core.color"
            :macros []
            :slot_fs_offset_color "void slot_fs_offset_color(inout vec4 outColor) {
                outColor.r = 1;
                outColor.g = 0;
                outColor.b = 0;
            }"
        }
    }
}