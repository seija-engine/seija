(add-backend "Camera" [
    {:name "projView" :type "mat4"  }
    {:name "view"     :type "mat4"  }
    {:name "proj"     :type "mat4"  }
    {:name "pos"      :type "float4"}
])

(add-backend "PerFrame" [
    {:name "positionFalloff" :type "float4"  }
    {:name "direction"     :type "float3"  }
    {:name "reserved1"     :type "float"  }
    {:name "pos"      :type "float4"}
])

;(def ubo-camera (node UBO_CAMERA {:backend "Camera"}))
;(def ubo-light (node UBO_LIGHT  {:backend "PerFrame"}))
;(def pass (node PASS))
;(def window (node WINDOW))

;(-> ubo-camera pass window)
;(-> ubo-light pass)