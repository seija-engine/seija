(require "all_uniform")


(defn declare-uniforms [set]
    (all_uniform/decl set)
)

(println "Enter Test Bloom Render Script")


(defn on-render-start [globalEnv]
    (println "on-render-start")
    (load-material "materials/bloom.mat.clj")

    (add-uniform  "ObjectBuffer")
    (add-uniform  "CameraBuffer")
    (add-uniform  "LightBuffer")
    

    (add-node globalEnv  CAMERA_NODE    "CameraBuffer")
    (add-node globalEnv  TRANSFROM_NODE "ObjectBuffer")
    (add-node globalEnv  PBR_CAMERA_EX  "CameraBuffer")
    (add-node globalEnv  PBR_LIGHT      "LightBuffer")

    (add-foward-path globalEnv)
)

(defn add-foward-path [globalEnv]
    (add-render-path "Foward" {
        :on-start (fn [env] 
            (assoc! env :depth (atom-texture {:format "Depth32Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (assoc! env :rawTexture  (atom-texture {:format "Bgra8Unorm" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (assoc! env :postEffect  (atom-texture {:format "Bgra8Unorm" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (assoc! env :postEffect2 (atom-texture {:format "Bgra8Unorm" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (add-node env  WINSIZE_TEXTURE [(env :depth) (env :targetView)])
            (add-node env  DRAW_PASS (env :camera-query) (env :camera-id) [(env :rawTexture)] (env :depth) "Foward")
            ;从rawTexture提取亮度到postEffect
            (add-node env  DRAW_QUAD "materials/bloom.mat.clj" [(env :postEffect)] (env :depth) [(env :rawTexture)] 0)
            ;从postEffect横向模糊到postEffect2
            (add-node env  DRAW_QUAD "materials/bloom.mat.clj" [(env :postEffect2)]  (env :depth) [(env :postEffect)] 1)
            ;从postEffect2纵向模糊到postEffect
            (add-node env  DRAW_QUAD "materials/bloom.mat.clj" [(env :postEffect)]  (env :depth) [(env :postEffect2)] 2)
            ;从postEffect和rawTexture合并到窗口
            (add-node env  DRAW_QUAD "materials/bloom.mat.clj" [(env :targetView)]  (env :depth) [(env :postEffect) (env :rawTexture)] 3)
            (println "add foward success")
        )
    })
)