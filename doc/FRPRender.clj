;uniform,node都是特殊的需要特殊都处理

(defn node-camera [uname] (node CAMERA_NODE uname))

(def-frp-comp base-3d-common []
  (uniform  "ObjectBuffer")
  (uniform  "CameraBuffer")
  (uniform  "LightBuffer" )
  (node-camera "CameraBuffer")
  (node TRANSFROM_NODE "ObjectBuffer")
  (node PBR_CAMERA_EX "CameraBuffer")
  (node PBR_LIGHT "LightBuffer")
)



(def-frp-comp start []
  (if-comp dynEnableBase3D '(base-3d-common ))
  (add-render-path foward-path-start)
)

(def-frp-comp foward-path-start [{:targetView window-texture :camera-id  camera-id :camera-query camera-query}]
   (let [depth-texture (texture {:format "Depth32Float"})]
      (node WINDOW_SIZE depth-texture)
      (if-comp dynIsHdr '(hdr-draw depth-texture window-texture camera-id camera-query)
                          '(normal-draw depth-texture window-texture camera-id camera-query))
   )
)

(def-frp-comp hdr-draw [depth-texture window-texture camera-id camera-query]
   (add-post-effect camera_id "tonemap")
   (let [hdr-texture (texture {:format "Rgba16Float"})] 
      (node DRAW_PASS camera_query camera_id [hdr-texture] depth-texture "Foward")
      (node POST_STACK camera_id hdr-texture window-texture)
   )
)

(def-frp-comp normal-draw [depth-texture window-texture camera-id camera-query]
   (if-comp dynHasPostEffect '(normal-draw-post-effect depth-texture window-texture camera-id camera-query)
                               (frp-comp #(node DRAW_PASS camera_query camera_id [window-texture] depth-texture "Foward"))
   )
)

(def-frp-comp normal-draw-post-effect [depth-texture window-texture camera-id camera-query]
   (let [cache-texture (texture {:format (get-format  window-texture)})]
      (node DRAW_PASS camera_query camera_id [cache-texture] depth-texture "Foward")
      (node POST_STACK camera_id cache-texture window-texture)
   )
)