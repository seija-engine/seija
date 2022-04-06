(add-ubo {
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

(add-ubo {
  :type :ComponentBuffer
  :apply :RenderObject
  :name "ObjectBuffer"
  :index 2
  :props [
     {:name "transform" :type "mat4"}
  ]
  :backends ["Transform"]
})

(add-ubo {
  :type :GlobalBuffer
  :apply :Frame
  :name "LightBuffer"
  :index 3
  :props [
     {:name "ambileColor"     :type "float4"}
     {:name "lightCount"      :type "int"}
     {:name "lights" :type [
        {:name "position"     :type "float3"}
        {:name "type"         :type "int"}
        {:name "direction"    :type "float3"}
        {:name "color"        :type "float3"}
        {:name "intensity"    :type "float"}
        {:name "ex1"          :type "float"}
        {:name "ex2"          :type "float"}
        {:name "ex3"          :type "float"}
     ] :size 10}
  ]
  :backends ["Light"]
})

(defn create-graph []
   (let [
            camera (node CAMERA    {:ubo "CameraBuffer" })
            transform (node TRANSFORM {:ubo "ObjectBuffer"})
            swapchain (node SWAP_CHAIN)
            pass (node PASS)
            depth-texture (node WINDOW_TEXTURE)
            light  (node LIGHT {:ubo "LightBuffer"})
        ]
        (link-> light          pass)
        (link-> transform      pass)
        (link-> camera         pass)
        (link-> swapchain      pass {0 0 1 2})
        (link-> depth-texture  pass {0 1})
   )
)