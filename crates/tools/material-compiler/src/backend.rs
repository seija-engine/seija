use std::{collections::HashMap, fmt::Write};
use glsl_pkg::IShaderBackend;

use crate::render_info::RenderInfo;

pub struct SeijaShaderBackend {
    pub render_info:RenderInfo,
    vertexs:HashMap<String,(usize,String)>
}

impl SeijaShaderBackend {
    pub fn init(&mut self,config_path:&str) {
        self.render_info.init();
        self.render_info.run(config_path);
    }
    pub fn new() -> Self {
        let mut vertexs = HashMap::new();
        vertexs.insert("POSITION".into(), (0,"vec3".into()));
        vertexs.insert("UV0".into(), (1,"vec2".into()));
        vertexs.insert("UV1".into(), (2,"vec2".into()));
        vertexs.insert("NORMAL".into(), (3,"vec3".into()));
        vertexs.insert("TANGENT".into(), (4,"vec3".into()));
        vertexs.insert("COLOR".into(), (5,"vec4".into()));
        SeijaShaderBackend { vertexs,render_info:RenderInfo::new() }
    }
}


impl IShaderBackend for SeijaShaderBackend {
    fn write_common_head<W:Write>(&self, writer:&mut W) {
        writer.write_str("#version 450\r\n").unwrap();
    }

    fn write_fs_head<W:Write>(&self, writer:&mut W) {
        writer.write_str("layout(location = 0) out vec4 _outColor;\r\n").unwrap();
      
    }

    fn vertex_names(&self) -> &HashMap<String,(usize,String)> {
       &self.vertexs
    }

    fn trait_fns<W:Write>(&self) -> HashMap<String, fn(&mut W)> {
        let mut traits:HashMap<String,fn(&mut W)> = HashMap::default();
        traits.insert("Camera".into(), get_camera_trait);
        traits.insert("Transform3D".into(), get_transform3d_trait);
        traits
    }

    fn write_uniforms<W:Write>(&self, writer:&mut W) {
        writer.write_str("layout(set = 0, binding = 0) uniform FrameUniforms {\r\n").unwrap();
        writer.write_str("  mat4 cameraVP;\r\n").unwrap();
        writer.write_str("  mat4 cameraView;\r\n").unwrap();
        writer.write_str("  mat4 cameraP;\r\n").unwrap();
        writer.write_str("  vec4 cameraPos;\r\n").unwrap();
        writer.write_str("} frameUniforms;\r\n").unwrap();

        writer.write_str("layout(set = 1, binding = 0) uniform ObjectUniforms {\r\n").unwrap();
        writer.write_str("  mat4 transform;\r\n").unwrap();
        writer.write_str("} objectUniforms;\r\n").unwrap();
       
    }
}

fn get_camera_trait<W:Write>(writer:&mut W) {
    writer.write_str("mat4 getCameraView() {\r\n").unwrap();
    writer.write_str("  return frameUniforms.cameraView;\r\n").unwrap();
    writer.write_str("}\r\n").unwrap();

    writer.write_str("mat4 getCameraViewProject() {\r\n").unwrap();
    writer.write_str("  return frameUniforms.cameraVP;\r\n").unwrap();
    writer.write_str("}\r\n").unwrap();
}

fn get_transform3d_trait<W:Write>(writer:&mut W) {
    writer.write_str("mat4 getObjectTransform() {\r\n").unwrap();
    writer.write_str("  return objectUniforms.transform;\r\n").unwrap();
    writer.write_str("}\r\n").unwrap();

   
}