## 上一版本的问题  
### 纹理数组的问题  
首先shader中有两种纹理数组。一种是`textureArray`,`textureArray`其实是一张纹理只是它可以在内部分层，类似于`cubemap`。  
另一种是`texure2D textures[8]`,这种才是我们想要的纹理的数组，其必须指定一个固定大小。  

首先`textureArray`肯定是不行的，因为它是一张纹理，要想模拟纹理数组的效果我们需要一次性申请 2048 * 2048 * 8 * 4这么大的内存，  
并且当数组的大小变化的时候，我们需要重新申请一块内存，把原来的数据拷贝过去，然后释放原来的内存。当图集个数非常多的时候，这个开销是非常大的。  


然后`texure2D textures[8]`,必须指定一个固定的大小，当图集个数变化的时候这个是不行的，除非写很多的宏变体，但是问题是不同宏变体的shader根本就不是一个pipeline了，也完全达不到同材质可以合批的效果。  
同时`texure2D textures[8]`还有非常多的其他的限制，首先`naga`的`glsl`前端目前根本就不支持这种写法。即使换用了`wgsl`,`wgsl`还有一个`ShaderNoUniform`没有支持，`ShaderNonUniform`是指在Shader中使用运行时可变索引（如访问数组或结构体成员）进行访问的能力。  

### 运行时动态加载的问题  
运行时动态加载会load出非常多的小Sprite散图，当要更新图集的时候.  
要么一次性全部刷新图集，这样会占有非常大的显存带宽。  
要么分成多次的command指定区域提交, 多次的command提交也是一个性能问题。  
除了上面说的提交GPU问题，频繁的读取小文件也是一个性能问题。  


## 新方案设计  

### 图集问题  
分析UGUI和FGUI后得出，他们的图集全部都是打包的时候打好的，运行时不会变化。   
区别就是FGUI是给图片打标签的方式，而UGUI是手动创建图集然后把散图拖到图集里。  
UGUI的图集使用完全是无感的，配合着编辑器，Image上只需要拖Sprite上去，图集的引用他会自动处理。  

考虑到UGUI的这种方案必须跟编辑器配合，目前只是实现Sprite指定图集模式，然后图集也是提前打好的。  

### 渲染合批问题  
之前的假设是所有渲染物体全在一个材质里，这里这个假设被打破需要重新设计。  

所以目前的前提是:  
1. UI元素按照树形的顺序排列，后面的元素会覆盖前面的元素。  
2. UI元素的材质是有可能不一样的。  
3. UI元素有些是动态的有些是静态的。  
4. UI元素有些情况下需要半透混合。  


#### Step1 
为了满足上面的前提，达到最大化合批的目的，其实需要非常多的CPU计算。   
所以首先设置一个实现前提，那就是静态的不会变化的UI元素可以耗费一些CPU合批。  
而动态的UI元素不进行任何合批计算，在实际应用中避免频繁的重建静态物体，这样才是比较合理的方案。  


#### Step2  
UI渲染的载体问题:   
UI渲染载体的目的就是对UI元素进行分组，不然直接对UI元素进行动静态标记就行了。  

假设一种UI场景，一个多个图片叠加的背景上有两个按钮。按钮有一个背景图和文本。  
第一个按钮点击的时候按钮的背景会切换。 第二个按钮点击的时候文本会切换并且文字会变色。  
```
Panel  
  BG0  
  BG1  
  Btn0 [Canvas]  
   BtnBG  
   Text  
  Btn1 [Canvas]  
   BtnBG  
   Text  
  StaticBtn0  
    BtnBG  
    Text  
  StaticBtn1  
    BtnBG  
    Text  
```
 按照树的顺序排序为:[BG0,BG1,StaticBtn0/BtnBG,StaticBtn0/Text,StaticBtn1/BtnBG,StaticBtn1/Text]
理想情况是有以下drawcall:
1. [BG0,BG1,StaticBtn0/BtnBG,StaticBtn1/BtnBG]
2. [Btn0/BtnBG]
3. [Btn0/Text]
4. [Btn1/BtnBG]
5. [Btn1/Text]
6. [StaticBtn0/Text,StaticBtn1/Text]

FGUI和UGUI都有通过检测Rect是否相交来达成1和6这种合批效果。  
相交检测算法的复杂度是O(n^2),这里的n是UI元素的数量。 因为他是双重for循环，如下:  
```C#
for(int i = 0; i < children.length; i++) {
  for(int j = i - 1; j >= 0; j--) {
    if(children[i].rect.intersects(children[j].rect)) {
        //dosomething
    }
  }
}
```
FGUI和UGUI不同的是，UGUI默认全部打开了这个功能但是他是16个元素为一组的。  
FGUI默认不是全部打开这个功能的,有些组件是默认打开的,有些可以手动指定某个Component是否打开，没有分组。  