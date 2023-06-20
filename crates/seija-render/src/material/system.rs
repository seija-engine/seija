use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use super::{Material, MaterialDef};
use crate::{
    memory::align_num_to,
    pipeline::render_bindings::{BindGroupBuilder, BindGroupLayoutBuilder},
    resource::{BufferId, RenderResources},
};
use bevy_ecs::{
    change_detection::Mut,
    entity::Entity,
    prelude::World, system::Resource,
};
use seija_asset::{Assets, Handle, HandleId};
use smol_str::SmolStr;
use wgpu::CommandEncoder;

pub struct MaterialSystem {
    common_buffer_layout: wgpu::BindGroupLayout,
    datas: HashMap<SmolStr, MaterialDefine>,
}

impl MaterialSystem {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut layout_builder = BindGroupLayoutBuilder::new();
        layout_builder.add_uniform(wgpu::ShaderStages::VERTEX_FRAGMENT);
        let common_buffer_layout = layout_builder.build(device);
        MaterialSystem {
            common_buffer_layout,
            datas: HashMap::default(),
        }
    }

    pub fn get_texture_layout(&self, name: &str) -> Option<&wgpu::BindGroupLayout> {
        self.datas.get(name).map(|v| &v.texture_layout)
    }

    fn add_material_define(&mut self, define: Arc<MaterialDef>, res: &mut RenderResources) {
        log::info!("add_material_define:{}",define.name.as_str());
        self.datas.insert(define.name.clone(), MaterialDefine::new(define, res));
    }

    pub fn get_buffer_layout(&self) -> &wgpu::BindGroupLayout {
        &self.common_buffer_layout
    }

    pub fn update(
        &mut self,
        world: &mut World,
        res: &mut RenderResources,
        commands: &mut CommandEncoder,
    ) {
        {
            let mut global_materials = world.get_resource_mut::<GlobalImportMaterials>().unwrap();
            if global_materials.is_dirty {
                for material_def in global_materials.material_defs.iter() {
                    if !self.datas.contains_key(material_def.name.as_str()) {
                        self.add_material_define(material_def.clone(), res);
                    }
                }
                global_materials.is_dirty = false;
            }
        };
        {
            let rm_list: Vec<Entity> = world.removed::<Handle<Material>>().collect();
            for define in self.datas.values_mut() {
                for rm_entity in rm_list.iter() {
                    //TODO 这里有bug
                    //define.remove_material(rm_entity)
                }
            }
        };
   

        let mut query = world.query::<(Entity, &Handle<Material>)>();
        world.resource_scope(|w, mut materials: Mut<Assets<Material>>| {
            //update material
            for (e, h_mat) in query.iter(w) {
                let h_mat: &Handle<Material> = h_mat;
                if let Some(mat) = materials.get_mut(&h_mat.id) {
                    if !self.datas.contains_key(mat.def.name.as_str()) {
                        self.add_material_define(mat.def.clone(), res);
                    }
                    
                    if let Some(define) = self.datas.get_mut(mat.def.name.as_str()) {
                        if mat.is_ready(res) {
                            mat.texture_props.update(res, Some(&define.texture_layout));
                        }

                        if !define.items.contains_key(&e) {
                           define.add_material(e, res,h_mat.id);
                        }

                        
                        define.update_bind_group(e, res, &self.common_buffer_layout, mat);
                    } 
                 
                }
            }

            for define in self.datas.values_mut() {
                if define.buffer_dirty {
                    for item in define.items.values().chain(define.free_items.iter()) {
                        if let Some(mat) = materials.get_mut(&item.mat_id) {
                            define.create_bind_group(mat, item.index, res, &self.common_buffer_layout);
                        }
                    }
                    define.buffer_dirty = false;
                }
                
                let mut cur_has_dirty = false;
                for item in define.items.values_mut().chain(define.free_items.iter_mut()) {
                    if item.is_dirty {
                        if let Some(mat) = materials.get_mut(&item.mat_id) {
                            

                            if cur_has_dirty == false {
                                res.map_buffer(&define.cache_buffer, wgpu::MapMode::Write);
                                cur_has_dirty = true;
                            }

                            if cur_has_dirty {
                                let start = item.index as u64 * define.buffer_item_size;
                                let buffer = mat.props.get_buffer();
                                res.write_mapped_buffer(
                                    &define.cache_buffer,
                                    start..(start + buffer.len() as u64),
                                    &mut |bytes, _| {
                                        bytes[0..buffer.len()].copy_from_slice(buffer);
                                    },
                                );
                                mat.props.clear_dirty();
                            }
                            
                        }
                        item.is_dirty = false;
                    }
                   
                    
                }
                if cur_has_dirty  {
                    res.unmap_buffer(&define.cache_buffer);
                    res.copy_buffer_to_buffer(
                        commands,
                        &define.cache_buffer,
                        0,
                        &define.gpu_buffer,
                        0,
                        define.cap as u64 * define.buffer_item_size,
                    );
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct DefineItem {
    index: usize,
    mat_id:HandleId,
    is_dirty:bool,
}

impl DefineItem {
    pub fn new(index: usize,mat_id:HandleId) -> Self {
        DefineItem {
            mat_id,
            index,
            is_dirty:true
        }
    }
}

#[derive(Debug)]
pub struct MaterialDefine {
    buffer_item_size: u64,
    cap: usize,
    len: usize,
    texture_layout: wgpu::BindGroupLayout,
    cache_buffer: BufferId,
    gpu_buffer: BufferId,

    items: fnv::FnvHashMap<Entity, DefineItem>,
    free_items: Vec<DefineItem>,

    buffer_dirty: bool,
}

impl MaterialDefine {
    pub fn new(define: Arc<MaterialDef>, res: &mut RenderResources) -> Self {
        let buffer_item_size: u64 =
            align_num_to(define.prop_def.size() as u64, res.device.limits().min_uniform_buffer_offset_alignment.into());
        let texture_layout = define.tex_prop_def.layout_builder.build(&res.device);
        let default_cap = 4;
        let (cache_buffer, gpu_buffer) = Self::alloc_buffer(default_cap, buffer_item_size, res);
        MaterialDefine {
            buffer_item_size,
            texture_layout,
            cache_buffer,
            gpu_buffer,
            cap: default_cap,
            len: 0,
            items: Default::default(),
            free_items: Default::default(),
            buffer_dirty: true,
        }
    }

    fn alloc_buffer(cap: usize, item_size: u64, res: &mut RenderResources) -> (BufferId, BufferId) {
        let cache_buffer = res.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: cap as u64 * item_size as u64,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
            mapped_at_creation: false,
        });

        let uniform_buffer = res.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: cap as u64 * item_size as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        (cache_buffer, uniform_buffer)
    }

    pub fn add_material(&mut self, id: Entity, res: &mut RenderResources,mat_id:HandleId) {
        if self.items.contains_key(&id) {
            return;
        }
        if let Some(free_item) = self.free_items.pop() {
            self.items.insert(id, free_item);
            return;
        }
        self.len += 1;
        if self.cap < self.len {
            while self.cap < self.len {
                self.cap *= 2;
            }
            
            let (new_cache_buffer, new_buffer) = Self::alloc_buffer(self.cap, self.buffer_item_size, res);
            self.cache_buffer = new_cache_buffer;
            self.gpu_buffer = new_buffer;
            self.buffer_dirty = true;
            for item in self.items.values_mut().chain(self.free_items.iter_mut()) {
                item.is_dirty = true;
            }
        }
        let index = self.len - 1;
        self.items.insert(id, DefineItem::new(index,mat_id));
    }

    pub fn remove_material(&mut self, id: &Entity) {
        if let Some(rm_item) = self.items.remove(id) {
            self.free_items.push(rm_item);
        }
    }

    pub fn update_bind_group(
        &mut self,
        id: Entity,
        res: &mut RenderResources,
        layout: &wgpu::BindGroupLayout,
        material: &mut Material
    ) {
        if let Some(item) = self.items.get_mut(&id) {
            if material.props.is_dirty() { item.is_dirty = true; }
            let item_index = item.index;
            if material.bind_group.is_none() {
                self.create_bind_group(material, item_index, res, layout);
            }
        }
    }

    pub fn create_bind_group(&self,material:&mut Material,item_index:usize,res: &mut RenderResources,layout: &wgpu::BindGroupLayout) {
        let mut build_group_builder = BindGroupBuilder::new();
        let start: u64 = item_index as u64 * self.buffer_item_size;
        build_group_builder.add_buffer_addr(self.gpu_buffer, start, self.buffer_item_size);
        material.bind_group = Some(build_group_builder.build(layout, &res.device, res));

    }
}


#[derive(Default,Resource)]
pub struct GlobalImportMaterials {
    material_defs:Vec<Arc<MaterialDef>>,
    name_set:HashSet<SmolStr>,
    is_dirty:bool
}

impl GlobalImportMaterials {
    pub fn add(&mut self,define:&Arc<MaterialDef>) {
        if !self.name_set.contains(define.name.as_str()) {
            self.is_dirty = true;
            self.material_defs.push(define.clone());
            self.name_set.insert(define.name.clone());
        }
    }

   
}