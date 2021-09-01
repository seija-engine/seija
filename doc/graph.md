1. Graph
     RenderObject
        Mesh
        Material
          props
            effNumber Int
            mainTexture Texture
            effTexture  Texture
          Stages
            Passes
              Pass
              
        


RenderScript.Create -> Material
```Clojure
  (def database
     [
        ["effNumber"   :Int]
        ["mainTexture" :Texture]
        ["effTexture"  :Texture]
     ]
  )

  
```
