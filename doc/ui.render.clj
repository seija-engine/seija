;  1. 在没有丰富的领域经验情况下，一个自上而下的整体设计非常困难
;  2. 可能会随着进度频繁的重构，推倒重写
;  3. 最好抓住最底层的概念，进行利于组合的设计，方便方向错误之后的快速反复重组。
;  4. 明确目标和已有概念，避免过多其他概念引入，造成过于问题杂乱，抓不清主次，把自己绕懵。

;基础概念:
;   Mesh,贴图, Buffer, Pipeline, Shader

;目标：实现渲染系统配置化
;    1. 加入RenderObject被渲染物体概念
;    2. RenderObject下加入Mesh和Material概念

