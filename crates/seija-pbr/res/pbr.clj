(require "core")

(defn add-pbr-camera-ubo [index]
    (let [props [{:name "exposure"  :type "float"  }]]
        (core/add-camera-ubo  index (concat core/core-camera-props props) ["PBRCameraEx"])
    )
)

(def add-pbr-light-ubo [index]
    (add-ubo {
        :type :GlobalBuffer
        :apply :Frame
        :name "LightBuffer"
        :index index
        :props [
           {:name "ambileColor"     :type "float4"}
           {:name "lightCount"      :type "int"}
           {:name "lights" :type [
              {:name "position"         :type "float3"}
              {:name "type"             :type "int"}
              {:name "direction"        :type "float3"}
              {:name "color"            :type "float3"}
              {:name "intensity"        :type "float"}
              {:name "falloff"          :type "float"}
              {:name "spotScaleOffset"  :type "float2"}
           ] :size 10}
        ]
        :backends ["PBRLight"]
    })
)

(defn create-pbr-graph []
    (let [
             camera        (node CAMERA           {:ubo "CameraBuffer" })
             pbr-camera-ex (node PBR_CAMERA_EX    {:ubo "CameraBuffer" })
             transform     (node TRANSFORM        {:ubo "ObjectBuffer" })
         ]
         (link-> camera pbr-camera-ex)
    )
 )