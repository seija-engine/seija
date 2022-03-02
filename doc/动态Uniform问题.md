## 设计需求
1. 渲染需要传递各种数据到shader中，目前除了材质外其他数据全部都是固定写死的。
2. 但是根据不同的渲染需求，会有不同的数据需要传递例如灯光数据，骨骼动画的数据等。
3. 需要一种可插拔的插件机制，负责数据的收集和传入。

## 一些前提
1. 一份uniform数据由 ”场景内实际的数据对象“  "收集后的容器" “GPU的BufferId和Layout”组成。
2. 希望 ”场景内实际的数据对象“ ->  "收集后的容器" 这一步在render graph中完成
3. 材质需要知道自己使用了哪些Uniform
4. 创建Pipeline的时候需要根据具体的shader的backend来引用对应的Uniform
5. 渲染的时候同上，需要知道怎么set Uniform

## 尝试1
1. "收集后的容器"和stage buffer存在graph node中。-> "CollectLight3DNode"
2. "GPU的BufferId和Layout"存在RenderContext上 -> "GPUUniformList"
3. 编译材质的时候导出一份shader对应的backend配置,并且加载到RenderContext上 -> "RuntimeShaderInfo"


## "GPUUniformList"需要支持什么
1. 支持PipelineCache创建pipeline时获取BindGroupLayout。根据PassDef的Shader.name
2. 支持RenderPipe渲染的时候按顺序设置GPUUniform
3. 支持GraphNode获取到对应的GPUUniform
4. 支持材质编译器根据Backend输出Shader (?)

## 尝试2
1. 使用`TypedUniform`，结构从配置文件中读取
2. `TypedUniform`和Stage Buffer存在Graph Node中 (Graph Node会使用脚本组装).
3. "GPU的BufferId和Layout"存在RenderContext上 -> "GPUUniformList"
4. 编译材质的时候导出一份shader对应的backend配置,并且加载到RenderContext上 -> "RuntimeShaderInfo"