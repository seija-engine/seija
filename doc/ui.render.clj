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
; filament使用PipelineKey作为一个Pipeline的key，会动态的根据这个Key创建或者获取已有的Pipeline
; struct PipelineKey {
;    VkShaderModule shaders[SHADER_MODULE_COUNT]; // 16 bytes
;    RasterState rasterState;                     // 124 bytes
;    VkPrimitiveTopology topology;                // 4 bytes
;    VkRenderPass renderPass;                     // 8 bytes
;    uint16_t subpassIndex;                       // 2 bytes
;    uint16_t padding0;                           // 2 bytes
;    VkVertexInputAttributeDescription vertexAttributes[VERTEX_ATTRIBUTE_COUNT]; // 256 bytes
;    VkVertexInputBindingDescription vertexBuffers[VERTEX_ATTRIBUTE_COUNT];      // 192 bytes
;    uint32_t padding1;                                                          // 4 bytes
;};  
; struct LightsUib {
    static constexpr utils::StaticString _name{ "LightsUniforms" };
    math::float4 positionFalloff;     // { float3(pos), 1/falloff^2 }
    math::half4 color;                // { half3(col),  0           }
    math::half4 directionIES;         // { half3(dir),  IES index   }
    math::half2 spotScaleOffset;      // { scale, offset }
    float intensity;                            // float
    uint32_t typeShadow;                        // 0x00.ll.ii.ct (t: 0=point, 1=spot, c:contact, ii: index, ll: layer)
    uint32_t channels;                          // 0x000c00ll (ll: light channels, c: caster)
    math::float4 reserved;            // 0

    static uint32_t packTypeShadow(uint8_t type, bool contactShadow, uint8_t index, uint8_t layer) noexcept {
        return (type & 0xF) | (contactShadow ? 0x10 : 0x00) | (index << 8) | (layer << 16);
    }
    static uint32_t packChannels(uint8_t lightChannels, bool castShadows) noexcept {
        return lightChannels | (castShadows ? 0x10000 : 0);
    }
};