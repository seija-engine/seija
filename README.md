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
        a. 需要对glsl进行进一步扩展

  6. 实现延迟渲染的渲染路径
  
  7. [可选] 梳理整个渲染管线以及材质系统，看是否可以进一步抽象，是否可以实现某种DSL简化结构。

Ⅱ. 接入脚本层
  1. 接入graalvm，使用Scala 
       不再考虑IOS禁止JIT和禁止加载未签名的代码导致的热更新问题。    
       graalvm可用通过嵌入Jre(JIT)，和native-image(AOT)两种方式执行。  
       因为使用jvm有了性能的保证，可以放心把一些底层的功能移到scala层来写。例如资源除存储之外的管理。

  3. 将API逐渐接入脚本层


Ⅲ. UI框架
  1. 实现基于数据绑定概念的UI框架 （经上几次失败尝试总结出FRP，elm like都不是合理解决方案. <del>函数式编程在UI领域狗屁不通</del>）。
  
  2. 实现基础控件库

Ⅳ. 自举游戏编辑器
  1. 游戏编辑器初期概念设计 (可能会参考emacs?)  
  ...  
   



<b>这次绝不弃坑!</b>
