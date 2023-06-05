# seija

开发路线  

Ⅰ. 渲染模块  
  1. <a href="./RENDER01.md"> 渲染内核 0.1</a>  
      <del>渲染内核 0.1为基础内核包括材质系统，资源管理系统，rdsl系统，材质编译系统，直接光照，阴影，和一些基础后处理，等所有基础功能。</del>  

  2. 渲染加强 V1 
    V1加强主要补充一些间接光照和后处理的一些技术。
     1. <del> 实现HDR和tonemap </del> 
     2. <del> 实现IBL </del> 
     3. 实现SSAO,TAA
     4. 考虑实现光照探针

     (考虑到进度问题渲染加强后续暂停,开始UI渲染层)  

Ⅱ. 实现一下Input功能  
   1. <del> 实现键盘接口 </del>    
   2. <del> 实现鼠标接口  </del> 

Ⅲ. UI渲染层  
   1. <del> 实现自动图集</del>
   2. <del>实现基于Panel机制的Mesh合并和渲染机制</del>  
   3. <del>实现Layout</del>  
   4. <del>实现UI事件系统</del>    
   5. <del> 实现文字渲染 </del>    
   6. 实现一些特殊控件渲染支持，例如Panel裁剪，Input控件等。
   7. <del>重构图集机制和Mesh合批，回归UGUI传统模式（以后谨慎听取没有相关领域经验的人胡扯技术方案）</del>  

Ⅳ. 接入脚本层
  1. <del> 接入scala native </del>
  2. <del> 进度进入上层语言scala native的功能实现，进度和路线查看 <a href="https://github.com/seija-engine/libseija-sn">libseija-sn</a> </del>   


Ⅴ. UI框架  
  <del> 进度进入上层语言scala native的功能实现，进度和路线查看 <a href="https://github.com/seija-engine/libseija-sn">libseija-sn</a> </del>  

Ⅵ. 自举游戏编辑器  
  <del> 进度进入上层语言scala native的功能实现，进度和路线查看 <a href="https://github.com/seija-engine/libseija-sn">libseija-sn</a> </del>
```
研究一下进程分离的编辑器模式  
如果可以的话把编辑器后端做成一个service，这样编辑器的前端就可以使用任意的技术堆栈，并且可以一个service连接多个显示前端。  
例如可以在vscode里直接连接编辑器后端做到一些智能的显示和扩展。
这样的话，就不只是局限于一个编辑器界面了，只要service可以连接的地方就是编辑器，或者叫他工作环境。 
```



<b>这次绝不弃坑,这次弃坑了我TM就是狗!</b>
