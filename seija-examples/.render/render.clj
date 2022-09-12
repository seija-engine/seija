(require "all_uniform")

(defn declare-uniforms [set]
    (all_uniform/decl set)
)

(println "Enter New Render Clojure")


(defn on-render-start [globalEnv]
    (println "on-render-start")
   

    (add-uniform  "ObjectBuffer")
    (add-uniform  "CameraBuffer")

    (add-uniform    "LightBuffer")
    (add-uniform   "SkinBuffer")
    

    (add-node globalEnv    CAMERA_NODE    "CameraBuffer")
    (add-node globalEnv    TRANSFROM_NODE "ObjectBuffer")
    (add-node globalEnv    PBR_CAMERA_EX  "CameraBuffer")
    (add-node globalEnv    PBR_LIGHT      "LightBuffer")
    
    (add-foward-path globalEnv)
)

(defn add-foward-path [globalEnv]
    (add-render-path "Foward" {
        :on-start (fn [env] 
            (assoc! env :depth (atom-texture {:format "Depth32Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (add-node env  WINSIZE_TEXTURE [(env :depth) (env :targetView)])
            (add-node env  DRAW_PASS (env :camera-query) (env :camera-id) [(env :targetView)] (env :depth) "Foward")
            (println "add foward success")
        )
    })
)

