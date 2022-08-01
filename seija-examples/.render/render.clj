(require "core")
(require "pbr")

(defn declare-uniforms [set]
   (core/declare-core-uniform   set)
   (pbr/declare-pbr-light      set 3)
   (core/declare-skin-uniform   set 4)
   (core/declare-shadow-uniform set 5)
)

(println "Enter New Render Clojure")


(defn on-render-start [globalEnv]
    (println "on-render-start")
    (add-tag "PBR" true)
    (add-tag "Skin" false)
    (add-tag "Shadow" true)

    (add-uniform  "ObjectBuffer")
    (add-uniform  "CameraBuffer")
    (select-add-uniform  "Shadow" "ShadowCast")
    (select-add-uniform  "Shadow" "ShadowRecv")

    (select-add-uniform  "PBR"    "LightBuffer")
    (select-add-uniform  "Skin"   "SkinBuffer")
    

    (add-node globalEnv nil   CAMERA_NODE    "CameraBuffer")
    (add-node globalEnv nil   TRANSFROM_NODE "ObjectBuffer")
    (add-node globalEnv "PBR" PBR_CAMERA_EX  "CameraBuffer")
    (add-node globalEnv "PBR" PBR_LIGHT      "LightBuffer")
    
    (if (tag? "Shadow")
        (do 
            (add-query "ShadowQuery" 2)
            (add-node globalEnv nil SHADOW_NODE "ShadowCast" "ShadowRecv")
            (assoc! globalEnv :shadowDepth (atom-texture {:format "Depth32Float" :width 4096 :height 4096}))
            (set-global-uniform "ShadowRecv" "shadowMap" (globalEnv :shadowDepth))
            (add-node globalEnv nil DRAW_PASS (get-query "ShadowQuery") nil [] (globalEnv :shadowDepth) "ShadowCaster")
        )
    )

    (add-foward-path globalEnv)
)

(defn add-foward-path [globalEnv]
    (add-render-path "Foward" {
        :on-start (fn [env] 
            (assoc! env :post-effect (atom-texture {:format "Bgra8UnormSrgb" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (assoc! env :depth (atom-texture {:format "Depth32Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (add-node env nil WINSIZE_TEXTURE [(env :depth) (env :targetView)])
            (add-node env nil DRAW_PASS (env :camera-query) (env :camera-id) [(env :post-effect)] (env :depth) "Foward")
            (println "add foward success")
        )
    })
)