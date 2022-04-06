## 前提
1. PBR的摄像机和灯光为了严谨的模拟物理信息，使用的参数和旧结构完全不同。  
   例如摄像机使用光圈，快门，ISO之类的控制曝光度，灯光使用辐射亮度，辐照度等单位。
   摄像机可能还涉及自动曝光和曝光修正等功能。

2. 需要运行在现有的render graph和动态uniform上。

## 需求  
1. 尽量在PBR和旧结构之间提取公用抽象，不做重复工作。
2. 保证动态的灵活引入，不能把一些东西耦合在一起，使用很小的功能确引入很多的代码。

## 问题
1. PBR的Camera和普通Camera怎么定义？
  1.  Camera + PBRCameraInfo -> camera_collect + pbr_camera_collect
  2.  Camera
         Box<dyn CameraDesc>  -> camera_collect + pbr_camera_collect | camera_collect + IDesc_writer