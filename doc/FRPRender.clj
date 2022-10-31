;uniform,node都是特殊的需要特殊都处理

(defn node-camera [uname] (node CAMERA_NODE uname))

(frp-comp base-3d-common []
  (uniform  "ObjectBuffer")
  (uniform  "CameraBuffer")
  (uniform  "LightBuffer" )
  (node-camera "CameraBuffer")
  (node TRANSFROM_NODE "ObjectBuffer")
  (node PBR_CAMERA_EX "CameraBuffer")
  (node PBR_LIGHT "LightBuffer")
)



(frp-comp start []
  (cond-comp dynEnableBase3D '(base-3d-common ))
  (add-render-path foward-path-start)
)

(frp-comp foward-path-start [{:targetView window-texture :camera-id  camera-id :camera-query camera-query}]
   (let [depth-texture (texture {:format "Depth32Float"})]
      (node WINDOW_SIZE depth-texture)
      (cond-comp dynIsHdr '(hdr-draw depth-texture window-texture camera-id camera-query)
                          '(normal-draw depth-texture window-texture camera-id camera-query))
   )
)

(frp-comp hdr-draw [depth-texture window-texture camera-id camera-query]
   (let [hdr-texture (texture {:format "Rgba16Float"})] 
      (node DRAW_PASS camera_query camera_id [hdr-texture] depth-texture "Foward")
      (node POST_STACK camera_id hdr-texture window-texture)
   )
)

(frp-comp normal-draw [depth-texture window-texture camera-id camera-query]

)