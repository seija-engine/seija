use glam::{Vec4, Vec3, Vec2};
use seija_render::{UniformBufferDef, UniformBuffer, IShaderBackend};

impl IShaderBackend for PBRLightBackend {
    fn from_def(def:&UniformBufferDef) -> Result<Self,String> where Self: Sized {
        PBRLightBackend::from_def(def)
    }

    fn set_count(&self,buffer:&mut UniformBuffer,count:i32) {
        self.set_light_count(buffer, count)
    }
}

pub struct PBRLightBackend {
    ambile_idx:usize,
    light_count_idx:usize,
    lights_type_idx:usize,
    lights_position_idx:usize,
    lights_item_size:usize,
    lights_direction_idx:usize,
    lights_color_idx:usize,
    lights_intensity_idx:usize,
    lights_falloff_idx:usize,
    lights_spot_scale_idx:usize,
    lights_spot_offset_idx:usize,
}

impl PBRLightBackend {
    pub fn from_def(def:&UniformBufferDef) -> Result<PBRLightBackend,String> {
        let ambile_idx = def.get_offset("ambileColor", 0).ok_or("ambileColor".to_string())?;
        let light_count_idx = def.get_offset("lightCount", 0).ok_or("lightCount".to_string())?;

        let lights_position_idx = def.get_array_offset("lights","position", 0).ok_or("lights.position".to_string())?;
        let lights_type_idx = def.get_array_offset("lights","type", 0).ok_or("lights.type".to_string())?;
        
        let lights_direction_idx = def.get_array_offset("lights","direction", 0).ok_or("lights.direction".to_string())?;
        let lights_color_idx = def.get_array_offset("lights","color", 0).ok_or("lights.color".to_string())?;
        let lights_intensity_idx = def.get_array_offset("lights","intensity", 0).ok_or("lights.intensity".to_string())?;
        let lights_falloff_idx = def.get_array_offset("lights","falloff", 0).ok_or("lights.falloff".to_string())?;
      
        let lights_spot_scale_idx = def.get_array_offset("lights","spotScale", 0).ok_or("lights.spotScale".to_string())?;
        let lights_spot_offset_idx = def.get_array_offset("lights","spotOffset", 0).ok_or("lights.spotOffset".to_string())?;


        let arr_info = def.get_array_info("lights").ok_or("ok_or".to_string())?;
        Ok(PBRLightBackend {
            ambile_idx,
            light_count_idx,
            lights_type_idx,
            lights_position_idx,
            lights_item_size:arr_info.stride,
            lights_direction_idx,
            lights_color_idx,
            lights_intensity_idx,
            lights_falloff_idx,
            lights_spot_scale_idx,
            lights_spot_offset_idx
        })
    }

    pub fn set_ambile_color(&self,buffer:&mut UniformBuffer,color:Vec4) {
        buffer.write_bytes(self.ambile_idx, color.to_array());
    }

    pub fn set_light_count(&self,buffer:&mut UniformBuffer,num:i32) {
        buffer.write_bytes(self.light_count_idx, num);
    }

    pub fn set_lights_position(&self,buffer:&mut UniformBuffer,index:usize,pos:Vec3) {
        let offset = self.lights_position_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, pos.to_array());
    }

    pub fn set_lights_type(&self,buffer:&mut UniformBuffer,index:usize,num:i32) {
        let offset = self.lights_type_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, num);
    }

    pub fn set_lights_direction(&self,buffer:&mut UniformBuffer,index:usize,dir:Vec3) {
        let offset = self.lights_direction_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, dir.to_array());
    }

    pub fn set_lights_color(&self,buffer:&mut UniformBuffer,index:usize,color:Vec3) {
        let offset = self.lights_color_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, color.to_array());
    }

    pub fn set_lights_intensity(&self,buffer:&mut UniformBuffer,index:usize,num:f32) {
        let offset = self.lights_intensity_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, num);
    }

    pub fn set_lights_falloff(&self,buffer:&mut UniformBuffer,index:usize,num:f32) {
        let offset = self.lights_falloff_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, num);
    }

    pub fn set_lights_spot_scale(&self,buffer:&mut UniformBuffer,index:usize,value:f32) {
        let offset = self.lights_spot_scale_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, value);
    }

    pub fn set_lights_spot_offset(&self,buffer:&mut UniformBuffer,index:usize,value:f32) {
        let offset = self.lights_spot_offset_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, value);
    }
}