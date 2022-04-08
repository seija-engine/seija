(require "core")
(require "pbr")

(pbr/add-pbr-camera-ubo 1)
(core/add-transform-ubo 2)

(defn create-graph []
    (pbr/create-pbr-graph)
)