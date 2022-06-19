{
    :name "pureColor"
    :order "Opaque"
    
    :props [
        {:name "color" :type "float4" :default [1,1,1,1]}
    ]
    :pass [
        {
            :shader { :name "core.color" :macros [] }
        }

        ;{
        ;    :tag "Shadow"
        ;    :shader { :name "core.depth" :macros [] }
        ;}
    ]
}