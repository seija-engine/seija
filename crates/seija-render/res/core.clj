(defn declare-core-uniform [set]
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

    (declare-uniform set "CameraBuffer" {
        :type :Component
        :apply :Camera
        :sort 2
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

(defn declare-skin-uniform [set index]
    (declare-uniform set "SkinBuffer" {
        :type :Component
        :sort index
        :apply :RenderObject
        :shader-stage SS_VERTEX
        :props [
            {:name "jointMats" :type "mat4[256]" }
        ]
        :backends ["SkinUniform"]
    })
)

(defn declare-posteffect-uniform [set index]
    (declare-uniform set "PostEffect" {
        :type :Component
        :sort index
        :apply :Camera
        :shader-stage SS_VERTEX_FRAGMENT
        :props []
        :textures [
            {
                :name "postTexture"
                :type "texture2D"
                :filterable true
            }
        ]
        :backends ["PostEffect"]
    })
)

(defn declare-shadow-uniform [set index]
    (declare-uniform set "ShadowCast" {
        :type :Global
        :sort index
        :apply :Frame
        :shader-stage SS_VERTEX
        :props [
            {:name "projView" :type "mat4" }
        ]
        :backends ["ShadowCast"]
    })

    (declare-uniform set "ShadowRecv" {
        :type :Global
        :sort (+ index 1)
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
                :filterable false
            }
        ]
        :backends ["ShadowRecv"]
    })
)

(defn declare-ibl-uniform [set index]
    (declare-uniform set "IBLEnv" {
        :type :Global
        :sort index
        :apply :Frame
        :shader-stage SS_FRAGMENT
        :props []
        :textures [
            { :name "irradianceMap" :type "cubeMap" :filterable true }
            { :name "prefilterMap" :type "cubeMap" :filterable true }
            { :name "brdfLUT" :type "texture2D" :filterable true }
        ]
        :backends ["IBLEnv"]
    })
)





