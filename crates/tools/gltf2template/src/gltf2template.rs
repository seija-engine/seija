use std::{env::current_dir, path::PathBuf, fs::File};
use glam::Mat4;
use gltf::{Gltf, json::Value, Material};
use anyhow::{Result, anyhow};
use relative_path::{RelativePath};
use crate::scheme::Scheme;
use xml::{self, EmitterConfig, writer::XmlEvent};

pub struct ExportConfig {
    pub relative_export:String
}
impl Default for ExportConfig {
    fn default() -> Self {
        ExportConfig { relative_export: "../template".into() }
    }
}

#[derive(Default)]
pub struct GlTF2Template {
    gltf_path:PathBuf,
    template_path:PathBuf,
    material_path:PathBuf,
    texture_path:PathBuf,
    textures:Vec<String>
}


impl GlTF2Template {
    pub fn run(&mut self,path:&str,config:&ExportConfig) -> Result<()> {
        let cur_dir = current_dir()?;
        let gltf_path:PathBuf = path.into();
        let file_stem = gltf_path.file_stem().map(|v| v.to_string_lossy()).ok_or(anyhow!("file_name is nil"))?;
        self.gltf_path = cur_dir.join(path).parent().ok_or(anyhow!("parent is nil"))?.into();
        self.template_path = RelativePath::new(&config.relative_export).join(file_stem.as_ref()).to_path(&self.gltf_path);
        self.texture_path = self.template_path.join("textures").into();
        self.material_path =self.template_path.join("mats").into();
       

        let gltf_data = Gltf::open(path)?;
        self.process_textures(&gltf_data)?;
        self.process_material(&gltf_data)?;
        self.process_template(&gltf_data)?;
        Ok(())
    }

    fn process_textures(&mut self,gltf:&Gltf) -> Result<()> {
        if gltf.textures().len() > 0 && !self.texture_path.exists() {
            std::fs::create_dir_all(&self.texture_path)?;
        }
        for json_texture in gltf.textures() {          
            let source = json_texture.source().source();
            match source {
                gltf::image::Source::View { view:_, mime_type:_ } => {
                    
                },
                gltf::image::Source::Uri { uri, mime_type } => {

                   match Scheme::parse(uri) {
                    Scheme::Data(_, _) => {
                       let _ex_type = mime_type.unwrap_or("png");
                       unimplemented!()
                    },
                    Scheme::File(file_path)  => {
                        let path_buf = PathBuf::from(file_path);
                        let move_to_path = self.texture_path.join(path_buf.file_name().unwrap());
                        std::fs::copy(&path_buf, &move_to_path)?;
                        self.textures.push(move_to_path.to_string_lossy().into());
                     },
                    Scheme::Relative => {
                        let cur_texture_path = self.gltf_path.join(uri);
                        let move_to_path = self.texture_path.join(cur_texture_path.file_name().unwrap());
                        std::fs::copy(cur_texture_path, &move_to_path)?;
                        let file_name = move_to_path.file_name().map(|v| v.to_string_lossy()).ok_or(anyhow!("err"))?;
                        self.textures.push(format!("textures/{}",file_name));
                    }
                    _ => {
                        log::error!("gltf texture error:{}",uri);
                        continue 
                    },
                   }
                }
            }
        }

        Ok(())
    }

    fn process_material(&mut self,gltf:&Gltf) -> Result<()> {
        if gltf.materials().len() > 0 && !self.material_path.exists() {
            std::fs::create_dir_all(&self.material_path)?;
        }
        for (index,material) in gltf.materials().enumerate() {
            let material_name = material.name().map(|v| v.to_string()).unwrap_or(format!("mat_{}",index));
            let json_value = self.into_material_to_json(&material)?;
            let cur_material_path = self.material_path.join(format!("{}.json",material_name));
            let json_string = serde_json::to_string_pretty(&json_value)?;
            std::fs::write(&cur_material_path, json_string.as_str())?;
        }

        Ok(())
    }

    fn into_material_to_json(&self,material:&Material) -> Result<Value>  {
        let mut mat_map:serde_json::Map<String,Value> = serde_json::Map::default();
        mat_map.insert("material".into(), "???..mat.clj".into());
        let mut props_map = serde_json::Map::default();
       
        
        let pbr_metallic_roughness = material.pbr_metallic_roughness();
        let base_color_factor = serde_json::to_value(pbr_metallic_roughness.base_color_factor())?;
        let metallic_factor = serde_json::to_value(pbr_metallic_roughness.metallic_factor())?;
        let roughness_factor = serde_json::to_value(pbr_metallic_roughness.roughness_factor())?;
        let emissive_factor = serde_json::to_value(material.emissive_factor())?;
        props_map.insert("baseColorFactor".into(), base_color_factor);
        props_map.insert("metallicFactor".into(), metallic_factor);
        props_map.insert("roughnessFactor".into(), roughness_factor);
        props_map.insert("emissiveFactor".into(), emissive_factor);
        if let Some(base_texture) = pbr_metallic_roughness.base_color_texture() {
           let texture_index = base_texture.texture().index();
           if let Some(path) = self.textures.get(texture_index) {
                props_map.insert("baseTexture".into(), path.as_str().into());
           }
        }
        if let Some(metallic_roughness_texture) = pbr_metallic_roughness.metallic_roughness_texture() {
            let texture_index = metallic_roughness_texture.texture().index();
            if let Some(path) = self.textures.get(texture_index) {
                 props_map.insert("metallicRoughnessTexture".into(), path.as_str().into());
            }
         }
         
         if let Some(number) = material.alpha_cutoff() {
            props_map.insert("alphaCutoff".into(), number.into());
         }

         if let Some(emissive_texture) = material.emissive_texture() {
            let texture_index = emissive_texture.texture().index();
            if let Some(path) = self.textures.get(texture_index) {
                 props_map.insert("emissiveTexture".into(), path.as_str().into());
            }
         }

         if let Some(occlusion_texture) = material.occlusion_texture() {
            let texture_index = occlusion_texture.texture().index();
            if let Some(path) = self.textures.get(texture_index) {
                 props_map.insert("occlusionTexture".into(), path.as_str().into());
            }
         }

        mat_map.insert("props".into(), props_map.into());
        Ok(Value::Object(mat_map))
    }

    fn process_template(&mut self,gltf:&Gltf) -> Result<()> {
        let mut gltf_name = self.gltf_path.file_stem().map(|v| v.to_string_lossy().to_string()).unwrap_or("template".into());
        gltf_name.push_str(".xml");
        let mut file = File::create(self.template_path.join(gltf_name))?;
        let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&mut file);
        

        file.sync_all()?;
        Ok(())
    }
}


fn get_gltf_meshs(gltf:&Gltf) {
    let mut all_nodes:Vec<Mat4> = vec![];
    for node in gltf.nodes() {
        match node.transform() {
            gltf::scene::Transform::Matrix { matrix } => {
               let mat = glam::Mat4::from_cols_array_2d(&matrix);
               all_nodes.push(mat);
            },
            gltf::scene::Transform::Decomposed { translation, rotation, scale } => {
               let p = glam::Vec3::from(translation);
               let r = glam::Quat::from_array(rotation);
               let s = glam::Vec3::from(scale);
               Mat4::from_scale_rotation_translation(s, r, p);
            }
        }
    }
    for scene in gltf.scenes() {
        for node in scene.nodes() {
            _get_gltf_meshs(&all_nodes,&node,&Mat4::IDENTITY);
        }
    } 
}

fn _get_gltf_meshs(all_nodes:&Vec<Mat4>,node:&gltf::Node,parent:&Mat4) {
    
}


#[test]
fn test_conv() {
    let mut opts = ExportConfig::default();
   
    let mut template = GlTF2Template::default();
    template.run("Fox/Fox.gltf", &opts).unwrap();
}