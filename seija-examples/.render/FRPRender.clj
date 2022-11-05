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
    (__frp_exit__)
)