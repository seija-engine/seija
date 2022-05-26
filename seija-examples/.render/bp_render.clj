(require "core")

(core/add-camera-ubo    1 [])

(core/add-transform-ubo 2)

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
     ] :size 64}
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