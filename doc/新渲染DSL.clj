(require "core")
(require "pbr")


(defn on-add-uniforms []
  (pbr/add-pbr-camera-ubo 1)
  (core/add-transform-ubo 2)
  (pbr/add-pbr-light-ubo  3)
  (core/add-anim-skin-ubo 4)
  (core/add-shadow-ubo 5)
)

(defn on-render-start [globalEnv]
    (env-add-texture :shadowMap globalEnv)
    
    (add-render-path "Deferred" {
        :on-start (fn [env] 
            (env-add-texture  :depth env {})
            (env-add-textures :gbufferTextures env [{} {} {} {}])
        )
        
        :on-update (fn [env]
            ;GBuffer
            (draw-pass (env :gbufferTextures) (env :depth) {:pass "GBuffer"})

            (draw-light-pass (env :gbufferTextures))

            (draw-pass (env :targetTexture) (env :depth) {:clear-depth false :pass "Foward"})
        )
    })
)

(defn on-render-update [globalEnv params]
    (camera-uniform-update "CameraBuffer"  )
    (transform-uniform-update "TransBuffer")
    (if (params :enable-pbr)
        (pbr-camera-update "CameraBuffer")
        (pbr-light-update "LightBuffer")
    )
    (draw-shadow-pass (globalEnv :shadowMap))
)


;;;;;;;;;;;;;;;;;;;;;;;;;;
(defn on-add-plugins []
    (add-plugin "Shadow" {
        :on-decl (fn []
             (declare-uniform set "ShadowCast" {
                :type :Global
                :sort index
                :apply :Frame
                :shader-stage SS_VERTEX
                :props [
                    {:name "projView" :type "mat4" }
                ]
                :backends ["ShadowCast"]
             })
        )

        :on-start (fn []
            
        )
    })
)

(defn on-decl []
    
)

(defn on-start []

)