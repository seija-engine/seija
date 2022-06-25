(require "core")
(require "pbr")

;现在这个只是给生成材质用的了
(defn declare-uniforms []
    (pbr/add-pbr-camera-ubo 1)
    (core/add-transform-ubo 2)
    (pbr/add-pbr-light-ubo  3)
    (core/add-anim-skin-ubo 4)
    (core/add-shadow-ubo 5)
)

(defn on-render-start [globalEnv]
    (core/add-transform-ubo 2)
    (pbr/add-pbr-camera-ubo 1)
    (nif "Skin"
        (core/add-anim-skin-ubo 4)    
    )
    (nif "Shadow"
        (core/add-shadow-ubo 5)
    )

    (add-render-path "Deferred" {
        :on-start (fn [env] 
            (env-add-texture  :depth env {})
            (env-add-textures :gbufferTextures env [{} {} {} {}])
            (create-quad (env :gbufferTextures) (env :depth))
        )
        
        :on-update (fn [env]
            ;GBuffer
            (draw-pass (env :gbufferTextures) (env :depth) {:pass "GBuffer"})
            (draw-pass (env :targetTexture) (env :depth) {:clear-depth false :pass "Foward"})
        )
    })
)

(defn on-render-update [globalEnv params]
    (camera-update "CameraBuffer"  )
    (transform-update "TransBuffer")
    (nif "PBR"
        (pbr-camera-update "CameraBuffer")
        (pbr-light-update "LightBuffer")
    )
    (draw-shadow-pass "Shadow")
)