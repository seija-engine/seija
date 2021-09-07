;  1. 在没有丰富的领域经验情况下，一个自上而下的整体设计非常困难
;  2. 可能会随着进度频繁的重构，推倒重写
;  3. 最好抓住最底层的概念，进行利于组合的设计，方便方向错误之后的快速反复重组。
;  4. 明确目标和已有概念，避免过多其他概念引入，造成过于问题杂乱，抓不清主次，把自己绕懵。

;基础概念:
;   Mesh,贴图, Buffer, Pipeline, Shader


;目标：实现渲染系统配置化
;    1. 加入RenderObject被渲染物体概念
;      1.1 RenderObject下加入
;          a. Mesh概念存储Mesh
;          b. MaterialDB: (存储贴图，存储Buffer类配置)
;          c. RenderScript: 完全动态？？？
;
;
;    2. 使用渲染图RenderGraph
;         2.1 通过配置构建graph.render
;         2.2  Node功能???
;
;
;  ========> 应该先实现一份基于render graph管线固定的材质系统，然后在考虑如何实现开发渲染管线的DSL。
;  
; 