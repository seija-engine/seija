(defn add-transform-ubo [index]
    (add-uniform {
        :type :Component
        :apply :RenderObject
        :name "ObjectBuffer"
        :index index
        :shader-stage SS_VERTEX
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
   (add-uniform {
        :type :Component
        :apply :Camera
        :name "CameraBuffer"
        :index index
        :shader-stage SS_VERTEX_FRAGMENT
        :props (if (nil? props) core/core-camera-props props)
        :backends (concat ["Camera3D"] backends)
   })    
)


(defn add-anim-skin-ubo [index]
    (add-uniform {
        :type :Component
        :name "SkinBuffer"
        :index index
        :apply :RenderObject
        :shader-stage SS_VERTEX
        :props [
            {:name "jointMats" :type "mat4[256]" }
        ]
        :backends ["SkinUniform"]
    })
)


(defn add-shadow-ubo [index]
    (add-uniform {
        :type :Global
        :name "ShadowMap"
        :index index
        :apply :Frame
        :shader-stage SS_FRAGMENT
        :props [
            {:name "lightProjView" :type "mat4" }
        ]
    
        ;:textures [
        ;    {
        ;        :name "shadowTexture"
        ;        :type "texture2D"
        ;        :filterable true
        ;    }
        ;]

        :backends ["ShadowMap"]
    })
)








