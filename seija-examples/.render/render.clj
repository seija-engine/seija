(require "core")
(require "pbr")

(pbr/add-pbr-camera-ubo 1)
(core/add-transform-ubo 2)
(pbr/add-pbr-light-ubo  3)
(core/add-anim-skin-ubo 4)
(defn create-graph []
   (let [
             camera        (node CAMERA           {:ubo "CameraBuffer" })
             pbr-camera-ex (node PBR_CAMERA_EX    {:ubo "CameraBuffer" })
             light         (node PBRLIGHT         {:ubo "LightBuffer"  })
             transform     (node TRANSFORM        {:ubo "ObjectBuffer" })
             swapchain     (node SWAP_CHAIN)
             depth-texture (node SCREEN_TEXTURE [{:format "Depth32Float"}])
             foward-pass  (node PASS {:is-depth true :clear-depth false})
         ]
         (link-> pbr-camera-ex camera)
         (link-> camera foward-pass)
         (link-> light  foward-pass)
         (link-> transform foward-pass)

         (link-> swapchain      foward-pass {0 0})
         (link-> depth-texture  foward-pass {0 1})
    )
)