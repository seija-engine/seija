(require "core")
(require "pbr")



(defn decl [set]
    (core/declare-core-uniform       set)
    (pbr/declare-pbr-light           set 3)
    (core/declare-skin-uniform       set 4)
    (core/declare-shadow-uniform     set 5)
    (core/declare-posteffect-uniform set 7)
    (core/declare-ibl-uniform        set 8)
    
)