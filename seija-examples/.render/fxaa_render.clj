(require "core")
(require "pbr")

(defn declare-uniforms [set]
   (core/declare-core-uniform set)
   (pbr/declare-pbr-light set 3)
)

(println "Enter New Render Clojure")


(defn on-render-start [globalEnv]
    (println "on-render-start")
    (load-material "res/materials/fxaa.mat.clj")

    (add-uniform  "ObjectBuffer")
    (add-uniform  "CameraBuffer")
    (add-uniform  "LightBuffer")
    

    (add-node globalEnv nil CAMERA_NODE    "CameraBuffer")
    (add-node globalEnv nil TRANSFROM_NODE "ObjectBuffer")
    (add-node globalEnv nil PBR_CAMERA_EX  "CameraBuffer")
    (add-node globalEnv nil PBR_LIGHT      "LightBuffer")

    (add-foward-path globalEnv)
)

(defn add-foward-path [globalEnv]
    (add-render-path "Foward" {
        :on-start (fn [env] 
            (assoc! env :depth (atom-texture {:format "Depth32Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (add-node env nil WINSIZE_TEXTURE [(env :depth) (env :targetView)])
            (add-node env nil DRAW_PASS (env :camera-query) (env :camera-id) [(env :targetView)] (env :depth) "Foward")
            (println "add foward success")
        )
    })
)