(defn declare-uniforms [set]
    (declare-uniform set "ObjectBuffer" {
        :type :Component
        :apply :RenderObject
        :sort 1
        :shader-stage SS_VERTEX
        :props [
           {:name "transform" :type "mat4"}
        ]
        :backends ["Transform"]
    })

    (declare-uniform set "LightBuffer" {
        :type :Global
        :apply :Frame
        :sort 2
        :shader-stage SS_FRAGMENT
        :props [
           {:name "ambileColor"     :type "float3"}
           {:name "lightCount"      :type "int"}
           {:name "lights" :type [
              {:name "position"         :type "float3"}
              {:name "type"             :type "int"}
              {:name "direction"        :type "float3"}
              {:name "color"            :type "float3"}
              {:name "intensity"        :type "float"}
              {:name "falloff"          :type "float"}
              {:name "spotScale"        :type "float"}
              {:name "spotOffset"       :type "float"}
           ] :size 4}
        ]
        :backends ["PBRLight"]
    })

    (declare-uniform set "CameraBuffer" {
        :type :Component
        :apply :Camera
        :sort 3
        :shader-stage SS_VERTEX_FRAGMENT
        :props [
            {:name "cameraView"       :type "mat4"  }
            {:name "cameraProj"       :type "mat4"  }
            {:name "cameraProjView"   :type "mat4"  }
            {:name "cameraPosition"   :type "float4"}
            {:name "exposure"  :type "float"  }
        ]
        :backends ["Camera3D" "PBRCameraEx"]
   })

   (declare-uniform set "SkinBuffer" {
        :type :Component
        :sort 4
        :apply :RenderObject
        :shader-stage SS_VERTEX
        :props [
            {:name "jointMats" :type "mat4[256]" }
        ]
        :backends ["SkinUniform"]
    })

    (declare-uniform set "ShadowCast" {
        :type :Global
        :sort 5
        :apply :Frame
        :shader-stage SS_VERTEX
        :props [
            {:name "projView" :type "mat4" }
        ]
        :backends ["ShadowCast"]
    })

    (declare-uniform set "ShadowRecv" {
        :type :Global
        :sort 6
        :apply :Frame
        :shader-stage SS_FRAGMENT
        :props [
            {:name "bias" :type "float" }
            {:name "strength" :type "float" }
        ]
        :textures [
            {
                :name "shadowMap"
                :type "texture2D"
                :filterable true
            }
        ]
        :backends ["ShadowRecv"]
    })
)

(println "Enter New Render Clojure")


(defn on-render-start [globalEnv]
    (println "on-render-start")
    (add-tag "PBR" true)
    (add-tag "Skin" false)
    (add-tag "Shadow" true)

    (add-uniform  "ObjectBuffer")
    (add-uniform  "CameraBuffer")
    (select-add-uniform  "Shadow" "ShadowCast")
    (select-add-uniform  "Shadow" "ShadowRecv")

    (select-add-uniform  "PBR"    "LightBuffer")
    (select-add-uniform  "Skin"   "SkinBuffer")
    

    (add-node globalEnv nil   CAMERA_NODE    "CameraBuffer")
    (add-node globalEnv nil   TRANSFROM_NODE "ObjectBuffer")
    (add-node globalEnv "PBR" PBR_CAMERA_EX  "CameraBuffer")
    (add-node globalEnv "PBR" PBR_LIGHT      "LightBuffer")

    (add-query "ShadowQuery" 2)
    (add-node globalEnv nil SHADOW_NODE "ShadowCast" "ShadowRecv")
    (assoc! globalEnv :shadowDepth (atom-texture {:format "Depth32Float" :width 2048 :height 2048}))
    (set-global-uniform "ShadowRecv" "shadowMap" (globalEnv :shadowDepth))
    (add-node globalEnv nil DRAW_PASS (get-query "ShadowQuery") nil [] (globalEnv :shadowDepth) "ShadowCaster")

    (add-foward-path globalEnv)
)

(defn add-foward-path [globalEnv]
    (add-render-path "Foward" {
        :on-start (fn [env] 
            (assoc! env :depth (atom-texture {:format "Depth32Float" :width WINDOW_WIDTH :height WINDOW_HEIGHT}))
            
            (add-node env nil WINSIZE_TEXTURE [(env :depth) (env :targetView)])

            (add-node env nil DRAW_PASS (env :camera-query) (env :camera-id) [(env :targetView)] (env :depth) "Foward")
            (println "add foward success")
        )
    })
)