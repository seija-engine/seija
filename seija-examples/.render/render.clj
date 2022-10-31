(require "all_uniform")

(defn declare-uniforms [set]
    (all_uniform/decl set)
)

(println "Enter New Render Clojure")

(defn on-render-start [globalEnv]
    (println "on-render-start")
   

    (add-uniform  "ObjectBuffer")
    (add-uniform  "CameraBuffer")

    (add-uniform  "LightBuffer")
    (add-uniform  "SkinBuffer")
    

    (add-node globalEnv    CAMERA_NODE    "CameraBuffer")
    (add-node globalEnv    TRANSFROM_NODE "ObjectBuffer")
    (add-node globalEnv    PBR_CAMERA_EX  "CameraBuffer")
    (add-node globalEnv    PBR_LIGHT      "LightBuffer")
    
    (add-render-path "Foward" {
        :on-start (fn [env] 
            (assoc! env :depth (atom-texture {:format "Depth32Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (assoc! env :hdr-texture (atom-texture {:format "Rgba16Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (let [
                   depth-texture (env :depth)
                   camera-id (env :camera-id) 
                   camera-query (env :camera-query)
                   camera-target (env :targetView)
                   hdr-texture (env :hdr-texture)
                 ]
                (add-node env  WINSIZE_TEXTURE [depth-texture camera-target])
                (add-node env  DRAW_PASS camera-query camera-id [hdr-texture] depth-texture "Foward")
                (add-node env  USE_POST_STACK camera-id hdr-texture camera-target)
                (println "add foward success")
            )
        )
    })
)

