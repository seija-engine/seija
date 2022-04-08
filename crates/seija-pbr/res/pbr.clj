(require "core")

(defn add-pbr-camera-ubo [index]
    (let [props [{:name "exposure"  :type "float"  }]]
        (core/add-camera-ubo  index (concat core/core-camera-props props) ["PBRCameraEx"])
    )
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