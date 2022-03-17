{
    :name "purecolor"
    :order "Opaque"
    :light true
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass {
        :shader {
            :name "core.color"
            :macros []
            :vert-process "void process(params) {

            }"
        }
    }
}