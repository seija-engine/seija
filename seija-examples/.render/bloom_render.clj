(require "all_uniform")


(defn declare-uniforms [set]
    (all_uniform/decl set)
)

(println "Enter Test Bloom Render Script")


(defn on-render-start [globalEnv]
    (println "on-render-start")
    (load-material "res/materials/bloom.mat.clj")

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
            (assoc! env :rawTexture  (atom-texture {:format "Bgra8UnormSrgb" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (assoc! env :postEffect  (atom-texture {:format "Bgra8UnormSrgb" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (assoc! env :postEffect2 (atom-texture {:format "Bgra8UnormSrgb" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (add-node env nil WINSIZE_TEXTURE [(env :depth) (env :targetView)])
            (add-node env nil DRAW_PASS (env :camera-query) (env :camera-id) [(env :rawTexture)] (env :depth) "Foward")
            ;从rawTexture提取亮度到postEffect
            (add-node env nil DRAW_QUAD "bloom" [(env :postEffect)] (env :depth) [(env :rawTexture)] 0)
            ;从postEffect横向模糊到postEffect2
            (add-node env nil DRAW_QUAD "bloom" [(env :postEffect2)]  (env :depth) [(env :postEffect)] 1)
            ;从postEffect2纵向模糊到postEffect
            (add-node env nil DRAW_QUAD "bloom" [(env :postEffect)]  (env :depth) [(env :postEffect2)] 2)
            ;从postEffect和rawTexture合并到窗口
            (add-node env nil DRAW_QUAD "bloom" [(env :targetView)]  (env :depth) [(env :postEffect) (env :rawTexture)] 3)
            (println "add foward success")
        )
    })
)