(def-ubo {
    :type :ComponentBuffer
    :apply :Camera
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
  :type :ComponentBuffer
  :apply :RenderObject
  :name "ObjectBuffer"
  :index 2
  :props [
     {:name "transform" :type "mat4"}
  ]
  :backends ["Transform"]
})

(def-ubo {
  :type :GlobalBuffer
  :apply :Frame
  :name "LightBuffer"
  :index 3
  :props [
     {:name "ambile" :type "float4"}
     {:name "lights" :type [
        {:name "type" :type "int"}
        {:name "position" :type "int"}
     ] :size 4}
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