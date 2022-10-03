(require "core")

(defrecord Base3D [name]
    (init [this set]
        (core/declare-core-uniform set)
    )

    (start [{dynEnable :dynEnable}]
        (uniform dynEnable "ObjectBuffer")
        (uniform dynEnable "CameraBuffer")
        (uniform dynEnable "LightBuffer")
    )

    (base3d-start-nodes [{dynEnable :dynEnable} env]
      
        (node dynEnable env  CAMERA_NODE    "CameraBuffer")
        (node dynEnable env  TRANSFROM_NODE "ObjectBuffer")
        (node dynEnable env  PBR_CAMERA_EX  "CameraBuffer")
        (node dynEnable env  PBR_LIGHT      "LightBuffer")            
    )
)


(defn init [set]
    (add-plugins [(Base3D. "Base3D") ])
    ;auto run plugin init
    
)

(defn create-depth32 [] (texture {:format "Depth32Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))

(defn create-dyn-target [eSetHDR]
    (let [normal-desc {:format "Depth32Float"}  hdr-desc {:format "Rgba32Float"}]
        (foldDyn normal eSetHDR #(if % normal hdr-desc))    
    )
    
)

(defn foward-path [env]
    (assoc! env :depth (create-depth32))
    (let [ 
            dynHdrDesc    (create-dyn-target eSetHDR) 
            targetTexture (dyn-texture dynHdrDesc)
         ]
        (add-node env  DRAW_PASS (env :camera-query) (env :camera-id) [targetTexture] (env :depth) "Foward")
        (posteffect-stack targetTexture (env :target-view))
    )
)

(defn start [env]
    (base3d-start-nodes [env])

    (.base3d-start-nodes (plugins "Base3D") env)

    (add-render-path "Foward" foward-path)
)