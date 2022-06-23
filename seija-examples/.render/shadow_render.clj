(require "core")
(require "pbr")

(pbr/add-pbr-camera-ubo 1)
(core/add-transform-ubo 2)
(pbr/add-pbr-light-ubo  3)
(core/add-anim-skin-ubo 4)
(core/add-shadow-ubo 5)
(defn create-graph []
   (let [
             camera        (node CAMERA           {:ubo "CameraBuffer" })
             pbr-camera-ex (node PBR_CAMERA_EX    {:ubo "CameraBuffer" })
             light         (node PBRLIGHT         {:ubo "LightBuffer"  })
             transform     (node TRANSFORM        {:ubo "ObjectBuffer" })
             
            
             gbuffer-pass  (node PASS {:is-outinput true :view-count 4 :is-depth true :path "Deferred"})
             swapchain     (node SWAP_CHAIN)
             depth-texture (node SCREEN_TEXTURE [{:format "Depth32Float"}])
             gbuffer-texs  (node SCREEN_TEXTURE [{:format "Rgba16Float"} 
                                                 {:format "Rgba16Float"}
                                                 {:format "Rgba8Unorm"}
                                                 {:format "Rgba8Unorm"} 
                                                 ])

             light-pass (node DEFERRED_LIGHT_PASS {:tex-count 4})
             foward-pass  (node PASS {:is-depth true :clear-depth false})

             shadow-map-texture (node SCREEN_TEXTURE [{:format "Depth32Float"}])
             shadow-map (node SHADOW_MAP {:ubo "ShadowMap"})
         ]
         (link-> shadow-map-texture shadow-map {0 0})
         (link-> pbr-camera-ex camera)
         (link-> camera gbuffer-pass)
         (link-> light  gbuffer-pass)
         (link-> transform camera)
         (link-> gbuffer-texs  gbuffer-pass {0 0 1 1 2 2 3 3})
         (link-> depth-texture gbuffer-pass {0 4})
        
         (link-> gbuffer-pass light-pass {0 0 1 1 2 2 3 3 4 4})

         (link-> light-pass     foward-pass)
         (link-> swapchain      foward-pass {0 0})
         (link-> depth-texture  foward-pass {0 1})
    )
)