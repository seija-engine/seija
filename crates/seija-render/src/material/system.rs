use bevy_ecs::prelude::{Entity, Mut, World};
use fnv::FnvHashMap;
use seija_asset::Handle;
use crate::{material::{storage::MaterialDefInfo}, pipeline::render_bindings::BindGroupLayoutBuilder, resource::{BufferId, RenderResources}};
use wgpu::{BufferUsage, CommandEncoder, Device};
use super::{Material, MaterialStorage};


pub struct MaterialSystem {
    buffers:FnvHashMap<String,BufferInfo>,
    pub material_layout:wgpu::BindGroupLayout,
    pub material_texture_layouts:FnvHashMap<String,wgpu::BindGroupLayout>
}

impl MaterialSystem {

    pub fn new(device:&Device) -> MaterialSystem {
        let mut material_layout_builder = BindGroupLayoutBuilder::new();
        material_layout_builder.add_uniform(wgpu::ShaderStage::VERTEX);
        MaterialSystem {
            buffers:fnv::FnvHashMap::default(),
            material_layout:material_layout_builder.build(device),
            material_texture_layouts:FnvHashMap::default()
        }
    }


    pub fn update(&mut self,world:&mut World,device:&Device,commands: &mut CommandEncoder,resources:&mut RenderResources) {
        world.resource_scope(|w,mut storage:Mut<MaterialStorage>| {
            self.update_material_texture_layout(&mut storage,device);
            self.update_material_props(w,device, storage,commands,resources);
        });
    }

    fn update_material_texture_layout(&mut self,storage:&mut Mut<MaterialStorage>,device:&Device) {
        let name_map_ref = storage.name_map.read();
        for (def_name,mat_def_info) in  name_map_ref.iter() {
            if self.material_texture_layouts.contains_key(def_name) { 
                continue; 
            }
            
            let layout = mat_def_info.def.texture_layout_builder.build(device);
            self.material_texture_layouts.insert(def_name.clone(), layout);
        }
    }
    
    fn update_material_props(&mut self,world:&mut World,device:&Device,storage:Mut<MaterialStorage>,commands: &mut CommandEncoder,resources:&mut RenderResources) {
        {
            let rm_list:Vec<Entity> = world.removed::<Handle<Material>>().collect();
            let name_map_ref = storage.name_map.read();
            for (def_name,mat_def) in  name_map_ref.iter() {
                let buffer = if let Some(buffer) = self.buffers.get_mut(def_name) {
                    buffer.update_size(mat_def.mat_count, resources);
                    buffer
                } else {
                    let new_buffer = BufferInfo::new(mat_def,resources);
                    self.buffers.insert(def_name.clone(), new_buffer);
                    self.buffers.get_mut(def_name).unwrap()
                };

                for rm_entity in rm_list.iter() {
                    buffer.remove_mat(rm_entity.id())
                }
            }
        }
        
        
        let mut query = world.query::<(Entity,&Handle<Material>)>();
        let mut mats = storage.mateials.write();
        for (e,mat_handle) in query.iter(world) {
            let mat_ref = mats.get_mut(&mat_handle.id).unwrap();
            if !mat_ref.is_ready(resources) {
                continue;
            }
            mat_ref.update(resources,device,&self.material_layout,self.material_texture_layouts.get(&mat_ref.def.name));
            if mat_ref.props.is_dirty() {
                let buffer_info = self.buffers.get_mut(&mat_ref.def.name).unwrap();  
                buffer_info.update(mat_ref, &e,resources,commands);
                mat_ref.props.clear_dirty();
            }   
        }
    }   
}



pub struct BufferInfo {
    item_size:usize,
    cur_count:usize,
    buffer:Option<BufferId>,
    len:usize,
    indices:FnvHashMap<u32,usize>,
    free_indices:Vec<usize>
}

impl BufferInfo {
    pub fn new(def_info:&MaterialDefInfo,resources:&mut RenderResources) -> BufferInfo {
        let mut info = BufferInfo {
            item_size:def_info.def.prop_def.size(), 
            cur_count:0, 
            buffer:None,
            indices:FnvHashMap::default(),
            free_indices:Vec::new(),
            len:0
         };
        
        info.update_size(def_info.mat_count,resources);
        info
    }

    pub fn remove_mat(&mut self,eid:u32) {
       if let Some(rm_idx) = self.indices.remove(&eid) {
           self.free_indices.push(rm_idx);
       }
    }

    pub fn update_size(&mut self,new_count:usize,resources:&mut RenderResources) {
        if self.cur_count > new_count || new_count == 0 {
            return;
        }
        self.cur_count = (((new_count as u32) + 3u32) & !3u32) as usize;
        let alloc_size = self.cur_count * self.item_size;
        
        let buffer = resources.create_buffer(&wgpu::BufferDescriptor { 
            label:None,
            size:alloc_size as u64,
            usage:BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
            mapped_at_creation:false 
        }); 
        self.buffer = Some(buffer);
    }

    pub fn update(&mut self,mat:&Material,e:&Entity,resources:&mut RenderResources,commands:&mut CommandEncoder) {
        let idx = self.get_or_insert_idx(e.id());
        let buffer_id = self.buffer.as_ref().unwrap();
        let start = idx * self.item_size;
        let end = start + self.item_size;
        resources.map_buffer(buffer_id, wgpu::MapMode::Write);

        resources.write_mapped_buffer(buffer_id, start as u64..end as u64, &mut |data,_| {
            data[0..self.item_size].copy_from_slice(mat.props.get_buffer());
        });

        resources.unmap_buffer(buffer_id);

        resources.copy_buffer_to_buffer(commands, 
                                          buffer_id, 
                                          start as u64, 
                                      mat.buffer.as_ref().unwrap(), 
                                      0, self.item_size as u64)
    }

    fn get_or_insert_idx(&mut self,eid:u32) -> usize {
        if let Some(idx) = self.indices.get(&eid) {
            *idx
        } else if self.free_indices.len() > 0 {
            let free_idx = self.free_indices.pop().unwrap();
            self.indices.insert(eid, free_idx);
            free_idx
        } 
        else  {
            let v = self.len;
            self.len += 1;
            self.indices.insert(eid, v);
            v
        }
    }
}