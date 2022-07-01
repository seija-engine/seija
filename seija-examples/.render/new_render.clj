(defn declare-uniforms [set]
    (declare-uniform set "ObjectBuffer" {
        :type :Component
        :apply :RenderObject
        :sort 1
        :shader-stage SS_VERTEX
        :props [
           {:name "transform" :type "mat4"}
        ]
        :backends ["Transform"]
    })

    (declare-uniform set "LightBuffer" {
        :type :Global
        :apply :Frame
        :sort 2
        :shader-stage SS_FRAGMENT
        :props [
           {:name "ambileColor"     :type "float3"}
           {:name "lightCount"      :type "int"}
           {:name "lights" :type [
              {:name "position"         :type "float3"}
              {:name "type"             :type "int"}
              {:name "direction"        :type "float3"}
              {:name "color"            :type "float3"}
              {:name "intensity"        :type "float"}
              {:name "falloff"          :type "float"}
              {:name "spotScale"        :type "float"}
              {:name "spotOffset"       :type "float"}
           ] :size 60}
        ]
        :backends ["PBRLight"]
    })
)

(println "Enter New Render Clojure")


(defn on-render-start [globalEnv]
    (println "on-render-start")
    (add-tag "PBR" true)
    (add-tag "Skin" false)
    (add-tag "Shadow" false)

    (add-uniform  "ObjectBuffer")
    ;(add-uniform  "CameraBuffer")
    (select-add-uniform  "PBR"    "LightBuffer")
    (select-add-uniform  "Skin"   "SkinBuffer")
    (select-add-uniform  "Shadow" "ShadowBuffer")

    (add-render-path "Deferred" {
        :on-start (fn [env] 
            (env-add-texture :depth env {})
            (env-add-textures :gbufferTextures env [{} {} {} {}])
        )
        
        :on-update (fn [env]
            ;GBuffer
            ;他妈的是谁忘了实现lambda语法糖
            (draw-pass (fn [] env :gbufferTextures) (fn [] env :depth) {:pass "GBuffer"})
            
            (draw-light-pass (env :gbufferTextures))

            (draw-pass (env :targetTexture) (env :depth) {:clear-depth false :pass "Foward"})
        )
    })
)

(defn on-render-update [globalEnv]
    (println " on-render-update")
    (assoc! globalEnv :testKey "ObjectBuffer")
    ;(camera-update "CameraBuffer")
    (transform-update (fn [] (globalEnv :testKey)))
    ;TMD我匿名函数语法糖呢
    ;(transform-update #(globalEnv :testKey) )

    ;(select-node "PBR" (do
    ;    (pbr-camera-update "CameraBuffer")
    ;    (pbr-light-update "CameraBuffer")    
    ;))
)