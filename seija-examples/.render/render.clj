(require "core")
(require "pbr")

(pbr/add-pbr-camera-ubo 1)
(core/add-transform-ubo 2)
(pbr/add-pbr-light-ubo  3)
(core/add-anim-skin-ubo 4)
(defn create-graph []
   (let [
             camera        (node CAMERA           {:ubo "CameraBuffer" })
             ;pbr-camera-ex (node PBR_CAMERA_EX    {:ubo "CameraBuffer" })
             ;light         (node PBRLIGHT         {:ubo "LightBuffer"  })
             transform     (node TRANSFORM        {:ubo "ObjectBuffer" })
             
            
             ;gbuffer-pass  (node PASS {:is-outinput true :view-count 2 :is-depth true :path "Deferred"})
             swapchain     (node SWAP_CHAIN)
             depth-texture (node SCREEN_TEXTURE [{:format "Depth32Float"}])
             ;gbuffer-texs  (node SCREEN_TEXTURE [{:format "Bgra8UnormSrgb"} {:format "Bgra8UnormSrgb"}])
             light-pass (node DEFERRED_LIGHT_PASS {:tex-count 0})
             foward-pass  (node PASS)
         ]
         (link-> camera foward-pass)
         ;(link-> light          gbuffer)
         (link-> transform      foward-pass)
         ;(link-> pbr-camera-ex  foward-pass)
         (link-> swapchain      foward-pass {0 0})
         (link-> depth-texture  foward-pass {0 1})

         ;(link-> swapchain      gbuffer-pass {0 1})
         ;(link-> gbuffer-texs   gbuffer-pass {1 0})
         ;(link-> depth-texture  gbuffer-pass {0 2})

         ;(link-> gbuffer-pass light-pass {0 0 1 1})
         
    )
    ;(pbr/create-pbr-graph true)
)