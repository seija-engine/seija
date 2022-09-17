```clojure
(defrecord ShadowPlugin []
   (init [this set]
      (decl-shadow-uniform set)
   )

   (start [this globalEnv]
      (add-uniform  "ShadowCast")
      (add-uniform  "ShadowRecv")
      (add-query "ShadowQuery" 2)
      (assoc! globalEnv :shadowDepth (atom-texture {:format "Depth32Float" :width 4096 :height 4096}))
      (set-global-uniform "ShadowRecv" "shadowMap" (globalEnv :shadowDepth))
      
      (do-tag this "Shadow" 
         (fn [globalEnv]
            (add-node globalEnv SHADOW_NODE "ShadowCast" "ShadowRecv")
            (add-node globalEnv DRAW_PASS (get-query "ShadowQuery") nil [] (globalEnv :shadowDepth) "ShadowCaster")
         )
      )
   )

   (exit [this globalEnv]
      (remove-uniform  "ShadowCast")
      (remove-uniform  "ShadowRecv")
      (remove-query "ShadowQuery" 2)
      (dissoc! globalEnv :shadowDepth)
   )
)

(defn on-init [set]
   (plugins [
      ;(Base3DPlugin.)
      (ShadowPlugin.)
   ])
)


(def on-start [globalEnv]
   (apply-nodes globalEnv [
      ;"Base3D"
      "Shadow"
   ])

   (add-render-path "Foward" {
        :on-start (fn [env] 
            (assoc! env :depth (atom-texture {:format "Depth32Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            (apply-nodes globalEnv [
                "BaseFoward"
            ])
        )
    })
)
```