## 什么引发的重构
  本质上来说是一个需求引起的重构即:“多摄像机，多渲染目标，多渲染路径共存”这个需求。  
  在多次尝试后发现<b>只用一个</b>Graph来描述渲染管线是完全错误的决定，同时也发现在有完整的渲染脚本的情况下Graph就是个鸡肋，下面依次解析。
### 只有一个Graph无法解决现有需求
  首先明确两个需求那就是  
  1. 摄像机可以可以指定渲染目标，这个渲染目标既可以是窗口Swapchain获取的Texture也可以是自己创建的RenderTexture。
  2. 摄像机可以指定一个渲染路径标识例如“Foward”或者"Deferred"，每个渲染路径对应一种渲染管线。    
其实把需求列出来很容易就能发现一个Graph基本无法完成需求。
。。。
为什么不用多个Graph呢，每个摄像机对应一个Graph。
。。。

### 为什么有渲染脚本Graph是鸡肋

## 新管线框架概述
### 动态标签和Uniform
### RenderPath

