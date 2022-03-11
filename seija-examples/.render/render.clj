(def-ubo {
    :type :PerCamera
    :name "CameraBuffer"
    :props [
        {:name "cameraView"       :type "mat4"  }
        {:name "cameraProj"       :type "mat4"  }
        {:name "cameraProjView"   :type "mat4"  }
        {:name "cameraPosition"   :type "float4"}
    ]
    :backends ["Camera3D"]
})

(def-ubo {
  :type :PerObject
  :name "ObjectBuffer"
  :props [
     {:name "transform" :type "mat4"}
  ]
})

(def camera (node CAMERA {:ubo "CameraBuffer" }))