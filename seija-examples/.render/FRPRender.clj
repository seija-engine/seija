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
  (let [depth-texture (texture {:format "Depth32Float"}) 
        dynIsHDR  (env :dynIsHDR)
        camera-id (env :camera-id)
        camera-query (env :camera-query)
        hdr-draw-comp [hdr-draw camera-id camera-query depth-texture] ]
    (node WinResizeNodeID [depth-texture])
    (if-comp dynIsHDR hdr-draw-comp)
  )
  (__frp_exit__)
)

(defn hdr-draw [camera-id camera-query depth-texture]
  (__frp_enter__ "hdr-draw")
  (let [hdr-texture (texture {:format "Rgba16Float"})]
    (node DrawPassNodeID camera-id camera-query [hdr-texture] depth-texture "Foward")
  )
  (__frp_exit__)
)


(add-render-path "Foward" foward-path)