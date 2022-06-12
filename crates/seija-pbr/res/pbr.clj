(require "core")

(defn add-pbr-camera-ubo [index]
    (let [props [{:name "exposure"  :type "float"  }]]
        (core/add-camera-ubo  index (concat core/core-camera-props props) ["PBRCameraEx"])
    )
)

(defn add-pbr-light-ubo [index]
    (add-uniform {
        :type :Global
        :apply :Frame
        :name "LightBuffer"
        :index index
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
)

(defn create-pbr-graph [is-skeleton]
    (let [
             camera        (node CAMERA           {:ubo "CameraBuffer" })
             pbr-camera-ex (node PBR_CAMERA_EX    {:ubo "CameraBuffer" })
             light         (node PBRLIGHT         {:ubo "LightBuffer"  })
             transform     (node TRANSFORM        {:ubo "ObjectBuffer" })
             swapchain     (node SWAP_CHAIN)
             pass          (node PASS)
             depth-texture (node WINDOW_TEXTURE)
         ]
         (link-> camera pbr-camera-ex)
         (link-> light          pass)
         (link-> transform      pass)
         (link-> pbr-camera-ex  pass)
         (link-> swapchain      pass {0 0 1 2})
         (link-> depth-texture  pass {0 1})
         (if is-skeleton
            (let [skeleton-skin (node SKELETON_SKIN {:ubo "SkinBuffer"})]
                (link-> skeleton-skin pass)
            )
         )
    )
 )