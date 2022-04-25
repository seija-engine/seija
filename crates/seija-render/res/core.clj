(defn add-transform-ubo [index]
    (add-ubo {
        :type :ComponentBuffer
        :apply :RenderObject
        :name "ObjectBuffer"
        :index index
        :props [
           {:name "transform" :type "mat4"}
        ]
        :backends ["Transform"]
    })
)

(def core-camera-props  [
    {:name "cameraView"       :type "mat4"  }
    {:name "cameraProj"       :type "mat4"  }
    {:name "cameraProjView"   :type "mat4"  }
    {:name "cameraPosition"   :type "float4"}
])

(defn add-camera-ubo [index props backends]
   (add-ubo {
        :type :ComponentBuffer
        :apply :Camera
        :name "CameraBuffer"
        :index index
        :props (if (nil? props) core/core-camera-props props)
        :backends (concat ["Camera3D"] backends)
   })    
)


(defn add-anim-skin-ubo [index]
    (add-ubo {
        :type :ComponentBuffer
        :name "SkinBuffer"
        :index index
        :apply :RenderObject
        :props [
            {:name "jointMats" :type "mat4[256]" }
        ]
        :backends ["SkinMat"]
    })
)
