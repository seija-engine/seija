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
        e. <del> 除了Uniform外还需要支持贴图的Backend </del> 

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
      c. <del> 实现之前设计漏实现的的material slot机制。</del>  
      

12. 实现后处理的结构和一些基础后处理效果  
      a. <del> 实现基于后处理的FXAA抗锯齿效果</del>    
      b. <del> 实现Bloom辉光效果 </del>  