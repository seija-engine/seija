## FRP组件  
### 概述  
FRP组件是FRP系统的基础。FRP组件以函数的方式声明，但他并不是传统的输入-输出的函数。 
`def-frp-comp`其实是一个Rust端的FRPNode的Builder。通过`def-frp-comp`的定义最终会构建出一个FRPNode的动态组件树。  
FRP系统里的基础概念`Event`,`Behavior`这里略过，参考`reflex-frp`。  

### 实现思路  
frp-comp函数其实就是在填充FRPCompBuilder.   
把FRPCompBuilder做成全局的动态变量。  
所有的FRP操作函数,像压栈一样Push进FRPCompBuilder,每个frp组件函数进入和退出时加入`__enter__`,`__exit__`来分割组件。  
最后用FRPCompBuilder的数据Build出最后的组件树。
```Clojure
;可以定义一个comp宏
;(fn [args]
;    (__frp_enter__ %fn-name)
;    (f args)
;    (__frp_exit__)
;)

(defn start [n]
  (__frp_enter__ "start")
  (uniform  "Name")
  (base-3d-common )
  (if (> n 0)
    (aa-comp-func )
    (bb-comp-func )
  )
  (if-comp  dynVar 
    #(if-a-comp) #(if-b-comp)
  )
  (__frp_exit__)
)

(defn base-3d-common []
 (__frp_enter__ "base-3d-common")
 (uniform  "222")
 (__frp_exit__)
)

```

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
### 组件嵌套   
子组件的嵌套只需要像调用函数一样在父组件的函数里调用就可以了.  
 
```Clojure
;m a -> (a -> m b) -> m b
(def-frp-comp start [n]
  ;普通函数调用
  (println "123")
  ;组件函数调用
  (base-3d-common )
  (if (> n 0)
    (aa-comp-func)
    (bb-comp-func)
  )
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