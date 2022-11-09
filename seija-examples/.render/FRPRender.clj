(require "all_uniform")

(defn init [set]
    (all_uniform/decl set)
)

(defn start []
    (__frp_enter__ "start")   
    (uniform "ObjectBuffer")
    (uniform "CameraBuffer")
    (uniform "LightBuffer")
    (node CameraNodeID "CameraBuffer")
    (node TransformNodeID "ObjectBuffer")
    (node PBRCameraExNodeID "CameraBuffer")
    (node PBRLightNodeID "LightBuffer")
    (__frp_exit__)
)


(defn foward-path [env]
  (__frp_enter__ "foward-path")
  (let [depth-texture (texture {:format "Depth32Float"}) hdr-draw-comp [hdr-draw 1 2]]
    (node WinResizeNodeID [depth-texture])
     ;(if-comp dynIsHDR hdr-draw-comp)
  )
  (__frp_exit__)
)

(defn hdr-draw [a b]

)


(add-render-path "Foward" foward-path)