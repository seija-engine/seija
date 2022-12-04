(require "all_uniform")

(defn init [set]
    (all_uniform/decl set)
)

(defcomp start []
    (uniform "ObjectBuffer")
    (uniform "CameraBuffer")
    (uniform "LightBuffer")
    (uniform "PostEffect")
    (uniform "IBLEnv")
    (node CameraNodeID "CameraBuffer")
    (node TransformNodeID "ObjectBuffer")
    (node PBRCameraExNodeID "CameraBuffer")
    (node PBRLightNodeID "LightBuffer")
    (node IBLNodeID "IBLEnv")
    (if-comp dynShadow [shadow-global])
)

(defcomp shadow-global []
  (uniform  "ShadowCast")
  (uniform  "ShadowRecv")
  (node ShadowNodeID "ShadowCast" "ShadowRecv")
  (let [shadow-query   (add-query "Shadow" 2)
        shadow-texture (texture {:format "Depth32Float" :width 4096 :height 4096})]
    (uniform-set nil "ShadowRecv" "shadowMap" shadow-texture)
    (node DrawPassNodeID shadow-query  nil  [] shadow-texture "ShadowCaster")
  )
)

(defcomp foward-path [env]
  (let [depth-texture (texture {:format "Depth32Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}) 
        dynIsHDR  (env :dynIsHDR)
        dynHasPostEffect  (env :dynHasPostEffect)
        camera-id (env :camera-id)
        camera-query (env :camera-query)
        camera-target (env :path-target)
        hdr-draw-comp [hdr-draw camera-id camera-query depth-texture camera-target]
        no-hdr-draw-comp [no-hdr-draw camera-id camera-query depth-texture camera-target dynHasPostEffect]
      ]
    (node WinResizeNodeID [depth-texture])
    (if-comp dynIsHDR 
              hdr-draw-comp 
              no-hdr-draw-comp
    )
  )
)

(defcomp hdr-draw [camera-id camera-query depth-texture camera-target]
  (posteffect-item camera-id "mats/tonemap.json" 1000)
  (let [hdr-texture (texture {:format "Rgba16Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT})]
    (node WinResizeNodeID [hdr-texture])
    (node DrawPassNodeID camera-query camera-id  [hdr-texture] depth-texture "Foward")
    (node PostStackNodeID camera-id hdr-texture camera-target)
  )
)

(defcomp no-hdr-draw [camera-id camera-query depth-texture camera-target dynHasPostEffect]
  (if-comp dynHasPostEffect
      [posteffect-draw camera-id camera-query depth-texture camera-target]
      [
        (fc [] (node DrawPassNodeID camera-query camera-id  [camera-target] depth-texture "Foward")) 
      ]
  )
)


(defcomp posteffect-draw [camera-id camera-query depth-texture camera-target]
  (let [cache-texture (texture {:format "Bgra8Unorm" :width WINDOW_WIDTH :height WINDOW_HEIGHT})]
    (node WinResizeNodeID [cache-texture])
    (node DrawPassNodeID camera-query camera-id  [cache-texture] depth-texture "Foward")
    (node PostStackNodeID camera-id cache-texture camera-target)
  )
)

(add-render-path "Foward" foward-path)