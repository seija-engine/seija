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
           ] :size 60}
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
)

(println "Enter New Render Clojure")


(defn on-render-start [globalEnv]
    (println "on-render-start")
    (add-tag "PBR" true)
    (add-tag "Skin" false)
    (add-tag "Shadow" false)

    (add-uniform  "ObjectBuffer")
    (add-uniform  "CameraBuffer")
    (select-add-uniform  "PBR"    "LightBuffer")
    (select-add-uniform  "Skin"   "SkinBuffer")
    (select-add-uniform  "Shadow" "ShadowBuffer")

    
    (add-node globalEnv nil   CAMERA_NODE    "CameraBuffer")
    (add-node globalEnv nil   TRANSFROM_NODE "ObjectBuffer")
    ;(add-node globalEnv "PBR" TRANSFROM_NODE "LightBuffer")
)

(defn add-foward-path [globalEnv]
    (add-render-path "Foward" {
        :on-start (fn [env] 
        
        )
    })
)