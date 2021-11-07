{:name "sky-box"
 :order "BeforeOpaque"
 :props [{:name "color" :type "float4" :default [1,1,1,1]}
         {:name "mainTexture" :type "CubeMap"}]
 :pass {:front-face "Cw"
        :z-write false
        :cull "Off"
        :vs "res/material/skybox/vert.spv"
        :fs "res/material/skybox/frag.spv"}}