(require "all_uniform")

(defn init [set]
    (all_uniform/decl set)
)

(def CameraNodeID 123)

(defn start []
    (__frp_enter__ "start")
    (uniform "ObjectBuffer")
    (uniform "CameraBuffer")
    (uniform "LightBuffer")
    (node CameraNodeID "CameraBuffer")
    (__frp_exit__)
)