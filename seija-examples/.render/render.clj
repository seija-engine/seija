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
             ;swapchain     (node SWAP_CHAIN)
             gbuffer          (node GBUFFER)
             depth-texture (node WINDOW_TEXTURE)
         ]
         (link-> camera pbr-camera-ex)
         (link-> light          gbuffer)
         (link-> transform      gbuffer)
         (link-> pbr-camera-ex  gbuffer)
         ;(link-> swapchain      gbuffer {0 0 1 2})
         (link-> depth-texture  gbuffer {0 0})
         
    )
    ;(pbr/create-pbr-graph true)
)