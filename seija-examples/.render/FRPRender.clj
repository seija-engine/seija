(require "all_uniform")

(defn init [set]
    (all_uniform/decl set)
)

(defn start []
    (__frp_enter__ "start")   
    (uniform "ObjectBuffer")
    (uniform "CameraBuffer")
    (uniform "LightBuffer")
    (uniform "PostEffect")
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
  (__frp_exit__)
)

(defn hdr-draw [camera-id camera-query depth-texture camera-target]
  (__frp_enter__ "hdr-draw")
  (let [hdr-texture (texture {:format "Rgba16Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT})]
    (node WinResizeNodeID [hdr-texture])
    (node DrawPassNodeID  camera-id camera-query [hdr-texture] depth-texture "Foward")
    (node PostStackNodeID camera-id hdr-texture camera-target)
  )
  (__frp_exit__)
)

(defn no-hdr-draw [camera-id camera-query depth-texture camera-target dynHasPostEffect]
  (__frp_enter__ "hdr-draw")
  (if-comp dynHasPostEffect
      [posteffect-draw camera-id camera-query depth-texture camera-target]
      [pure-draw camera-id camera-query depth-texture camera-target]
  )
  (__frp_exit__)
)

(defn posteffect-draw [camera-id camera-query depth-texture camera-target]
  (__frp_enter__ "posteffect-draw")
  (let [cache-texture (texture {:format "Bgra8Unorm" :width WINDOW_WIDTH :height WINDOW_HEIGHT})]
    (node WinResizeNodeID [cache-texture])
    (node DrawPassNodeID  camera-id camera-query [cache-texture] depth-texture "Foward")
    (node PostStackNodeID camera-id cache-texture camera-target)
  )
  (__frp_exit__)
)

(defn pure-draw [camera-id camera-query depth-texture camera-target]
  (__frp_enter__ "pure-draw")
  (node DrawPassNodeID  camera-id camera-query [camera-target] depth-texture "Foward")
  (__frp_exit__)
)

(add-render-path "Foward" foward-path)