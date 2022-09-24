(require "all_uniform")

(defrecord Base3D []
    
    (init []
      (println "Base3D Init") 
      (add-node-list "Base3D" (fn [env]
         (println "run Base3D node list")
         (add-node env  CAMERA_NODE    "CameraBuffer")
         (add-node env  TRANSFROM_NODE "ObjectBuffer")
         (add-node env  PBR_CAMERA_EX  "CameraBuffer")
         (add-node env  PBR_LIGHT      "LightBuffer")
      ))
    )

    (start []
        (println "Base3D Start")
        (add-uniform  "ObjectBuffer")
        (add-uniform  "CameraBuffer")
        (add-uniform  "LightBuffer")
    )
)


(defn declare-uniforms [set]
    (all_uniform/decl set)
    (plugins [
       (Base3D. ) 
    ])
)



(defn on-render-start [globalEnv]
    (println "on-render-start")
    (load-material "materials/fxaa.mat.clj")
    (node-list globalEnv ["Base3D"])
    (add-foward-path globalEnv)
)

(defn add-foward-path [globalEnv]
    (add-render-path "Foward" {
        :on-start (fn [env] 
            (assoc! env :depth (atom-texture {:format "Depth32Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            ;(assoc! env :postEffect (atom-texture {:format "Bgra8Unorm" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            
            ;(add-node env nil WINSIZE_TEXTURE [(env :depth) (env :targetView)])
            (add-node env  DRAW_PASS (env :camera-query) (env :camera-id) [(env :targetView)] (env :depth) "Foward")

            ;(add-node env nil DRAW_QUAD "materials/fxaa.mat.clj" [(env :targetView)] (env :depth) [(env :postEffect)])
            (println "add foward success")
        )
    })
)