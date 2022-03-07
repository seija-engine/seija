(def-ubo {
    :type :PerCamera
    :name "CameraBuffer"
    :props [
        {:name "projView"  :type "mat4"  }
        {:name "view"      :type "mat4"  }
        {:name "proj"      :type "mat4"  }
        {:name "position"  :type "float4"}
    ]
    :backends ["Camera3D"]
})

(def collect-camera (node CAMERA {:ubo "CameraBuffer" }))
(-> camera swap)
(-> depth pass {0 1})

;(def ubo-light (node UBO_LIGHT  {:backend "PerFrame"}))
;(def pass (node PASS))
;(def window (node WINDOW))

;(-> ubo-camera pass window)
;(-> ubo-light pass)