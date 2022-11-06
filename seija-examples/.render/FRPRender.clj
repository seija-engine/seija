(require "all_uniform")

(defn init [set]
    (all_uniform/decl set)
)

(println CameraNodeID)

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


(defn foward-path []
  (__frp_enter__ "foward-path")
  
  (__frp_exit__)
)


(add-render-path "Foward" foward-path)