(def-ubo {
    :type :PerCamera
    :name "CameraBuffer"
    :index 1
    :props [
        {:name "cameraView"       :type "mat4"  }
        {:name "cameraProj"       :type "mat4"  }
        {:name "cameraProjView"   :type "mat4"  }
        {:name "cameraPosition"   :type "float4"}
    ]
    :backends ["Camera3D"]
})

(def-ubo {
  :type :PerObject
  :name "ObjectBuffer"
  :index 2
  :props [
     {:name "transform" :type "mat4"}
  ]
  :backends ["Transform"]
})

(def-ubo {
  :type :PerObject
  :name "LightBuffer"
  :index 3
  :props [
     {:name "positionFalloff" :type "float4"}
     {:name "direction"       :type "float3"}
  ]
  :backends ["Light"]
})

(def camera    (node CAMERA    {:ubo "CameraBuffer" }))
(def transform (node TRANSFORM {:ubo "ObjectBuffer"}))
(def swapchain (node SWAP_CHAIN))
(def pass (node PASS))
(def depth-texture (node WINDOW_TEXTURE))

(link-> transform      pass)
(link-> camera         pass)
(link-> swapchain      pass {0 0})
(link-> depth-texture  pass {0 1})