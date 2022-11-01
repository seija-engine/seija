## FRP组件  
### 概述  
FRP组件是FRP系统的基础。FRP组件以函数的方式声明，但他并不是传统的输入-输出的函数。 
`def-frp-comp`其实是一个Rust端的FRPNode的Builder。通过`def-frp-comp`的定义最终会构建出一个FRPNode的动态组件树。  
FRP系统里的基础概念`Event`,`Behavior`这里略过，参考`reflex-frp`。  


### 基础元素
FRP组件的生成，就是为了生成一些基础元素到FRPNode上。  
这些元素包括 
1. `Uniform` : GPU需要的数据。 
2. `Texture`: 贴图数据，生命周期跟随FRP组件
2. `Node` : 动作节点,FRP的基础组成之一，用来收集更新数据,执行实际渲染等一系列需要Update操作。  

FRP组件激活和取消激活都会有对应的响应函数。响应函数同时会调用组件下所有元素的激活和取消激活。    
```Clojure
(def-frp-comp base-3d-common []
  (uniform  "ObjectBuffer")
  (uniform  "CameraBuffer")
  (uniform  "LightBuffer" )
  (node CAMERA_NODE "CameraBuffer")
)
```

### FRP的组件动态处理  
1. <b>if-comp</b>开关组件  
     第一个参数接收一个`Dynamic Bool`表示组件的开关.  
     第二个参数接收组件构建函数。  
     根据外部`Dynamic Bool`的变化为true这个组件会被激活,为false会被关闭  
     第三个参数可选`Dynamic Bool`为false的时候激活，为true的时候关闭
```Clojure
(def-frp-comp start []
  (if-comp (const-dyn true) '(base-3d-common))
)
```

2. 