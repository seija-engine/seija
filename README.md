# seija

开发路线  

Ⅰ. 渲染核心实现
  1. 实现基础材质系统  
        a. <del> 完成PipelineCache的Pipeline构建工作 </del>  
        b. <del> 修改摄像机的Buffer存储 </del>  
        c. <del> 完成PassNode </del>  
        d. <del> 绘制出第一个Cube </del>  
        e. <del> 完成材质属性配置中贴图的支持 </del>


  2. 完成gltf模型渲染  
        a. <del> 完成Mesh加载和测试渲染 </del>  
        b. <del> 完成材质Scene和Node的加载 </del>  
        c. <del> 根据gltf文件创建渲染对象(根据Scene,Node,Mesh创建对应元素) </del>

  3. 完成前向渲染的基础光照  
        a. <del>先实现一个环境贴图 </del>  
        b. <del>实现基础光源的数据结构和Buffer传递</del>  
        c. <del>实现Phong和Blinn Phong光照 </del>  
  
  4. 实现PBR体系以及PBR相关内建Shader  
        a. <del> 完成PBR Cook-Torrance的BRDF的高光和漫反射部分。 </del>  

  5. 进一步优化材质系统  
        a. <del> 实现贴图属性的默认值 </del>  
        b. <del> 完成glsl-pack </del>  
        c. <del> 完成材质编译工具 </del>   
        d. <del> 重新整理uniform的结构，做成插件模式 </del>  
        e. 除了Uniform外还需要支持贴图的Backend

  6. 使用新的材质系统重新梳理实现各种光照shader  
        a. <del> 重新实现Blinn-Phong光照模型和相关灯光处理</del>  
        b. <del> 重新实现PBR光照模型和相关灯光数据(物理严格)</del>  

  7. 实现骨骼动画  
        a. <del>实现骨骼动画相关数据结构和动画采样</del>  
        b. <del>实现gltf的动画加载</del>  
        c. <del>实现骨骼动画的graph node和相关渲染shader和配置</del>  
      
  8. 实现延迟渲染的渲染路径  
      a. <del>实现GBuffer阶段</del>  
      b. <del>实现LightPass阶段</del>  
      c. <del>完成延迟渲染的显示</del>

  9. 支持延迟渲染和前向渲染共存  
      a. <del>兼容深度问题 </del>  

  10. 实现一下各种阴影效果  
      a. <del> 实现阴影深度贴图渲染 </del>   
      b. <del> 实现普通ShadowMap阴影效果 </del>  

  11. 进行最后一次渲染管线框架重构  
      a. <del> 为了解决多摄像机，多渲染路径，多渲染目标问题。</del>   
      b. <del> 为了解决uniform和渲染节点在运行时动态的添加删除问题。 </del>    
      

  12. 实现后处理的结构和一些基础后处理效果  
      a. <del> 实现基于后处理的FXAA抗锯齿效果</del>    
      b. <del> 实现Bloom辉光效果 </del>  
  

  13. 找资源做一个渲染demo展示渲染效果，然后完成渲染部分开发  

  
  14. 清理遗留的TODO    

Ⅱ. 实现一下Input功能  

Ⅲ. 接入脚本层
  1. 接入graalvm，使用Scala 
       不再考虑IOS禁止JIT和禁止加载未签名的代码导致的热更新问题。    
       graalvm可用通过嵌入Jre(JIT)，和native-image(AOT)两种方式执行。  
       因为使用jvm有了性能的保证，可以放心把一些底层的功能移到scala层来写。例如资源除存储之外的管理。

  3. 将API逐渐接入脚本层


Ⅳ. UI框架
  1. 实现基于数据绑定概念的UI框架 （经上几次失败尝试总结出FRP，elm like都不是合理解决方案. <del>函数式编程在UI领域狗屁不通</del>）。
  
  2. 实现基础控件库

Ⅴ. 自举游戏编辑器
  1. 研究一下进程分离的编辑器模式  
       如果可以的话把编辑器后端做成一个service，这样编辑器的前端就可以使用任意的技术堆栈，并且可以一个service连接多个显示前端。  
       例如可以在vscode里直接连接编辑器后端做到一些智能的显示和扩展。
       这样的话，就不只是局限于一个编辑器界面了，只要service可以连接的地方就是编辑器，或者叫他工作环境。 
   



<b>这次绝不弃坑,这次弃坑了我TM就是狗!</b>
