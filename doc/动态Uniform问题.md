## 设计需求
1. 渲染需要传递各种数据到shader中，目前除了材质外其他数据全部都是固定写死的。
2. 但是根据不同的渲染需求，会有不同的数据需要传递例如灯光数据，骨骼动画的数据等。
3. 需要一种可插拔的插件机制，负责数据的收集和传入。

## 一些前提
1. 一份uniform数据由 ”场景内实际的数据对象“  "收集后的容器" “GPU的BufferId和Layout”组成。
2. 希望 ”场景内实际的数据对象“ ->  "收集后的容器" 这一步在render graph中完成
3. 材质需要知道自己使用了哪些Uniform

## 尝试1
1. "收集后的容器"和stage buffer存在graph node中
2. “GPU的BufferId和Layout”存在RenderContext上
3. 创建Pipeline的时候需要根据具体的shader的backend来引用对应的Uniform
4. 渲染的时候同上，需要知道怎么set Uniform
