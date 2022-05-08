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
             ;light         (node PBRLIGHT         {:ubo "LightBuffer"  })
             transform     (node TRANSFORM        {:ubo "ObjectBuffer" })
             
            
             gbuffer-pass  (node PASS {:view-count 2 :is-depth true :path "Deferred"})
             swapchain     (node SWAP_CHAIN)
             depth-texture (node SCREEN_TEXTURE [{:format "Depth32Float"}])
             gbuffer-texs  (node SCREEN_TEXTURE [{:format "Bgra8UnormSrgb"} {:format "Bgra8UnormSrgb"}])
         ]
         (link-> camera pbr-camera-ex)
         ;(link-> light          gbuffer)
         (link-> transform      gbuffer-pass)
         (link-> pbr-camera-ex  gbuffer-pass)
         (link-> swapchain      gbuffer-pass {0 1})
         (link-> gbuffer-texs   gbuffer-pass {1 0})
         (link-> depth-texture  gbuffer-pass {0 2})
         
    )
    ;(pbr/create-pbr-graph true)
)