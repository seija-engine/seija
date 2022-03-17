(def-ubo {
    :type :PerCamera
    :name "CameraBuffer"
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
  :props [
     {:name "transform" :type "mat4"}
  ]
  :backends ["Transform"]
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