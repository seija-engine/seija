use std::{collections::HashMap,sync::Arc};
use super::{Material, MaterialDef};
use crate::{memory::align_num_to,pipeline::render_bindings::{BindGroupBuilder, BindGroupLayoutBuilder},resource::{BufferId, RenderResources}};
use bevy_ecs::{
    change_detection::Mut,
    entity::Entity,
    prelude::World, event::{ManualEventReader, Events}
};
use seija_asset::{Assets, Handle, HandleId, AssetEvent};
use smol_str::SmolStr;
use wgpu::CommandEncoder;

#[derive(Debug)]
pub struct DefineItem {
    index: usize,
    bind_group:Arc<wgpu::BindGroup>,
    h_material:Option<HandleId>,
    is_dirty:bool,
}

#[derive(Debug)]
pub struct MaterialDefine {
    buffer_item_size: u64,
    cap: usize,
    len: usize,
    texture_layout: wgpu::BindGroupLayout,
    cache_buffer: BufferId,
    gpu_buffer: BufferId,

    items: fnv::FnvHashMap<HandleId, DefineItem>,
    free_items: Vec<DefineItem>,

    buffer_dirty: bool,
}


pub struct MaterialSystem {
    common_buffer_layout: Arc<wgpu::BindGroupLayout>,
    datas: HashMap<SmolStr, MaterialDefine>,
}

impl MaterialSystem {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut layout_builder = BindGroupLayoutBuilder::new();
        layout_builder.add_uniform(wgpu::ShaderStages::VERTEX_FRAGMENT);
        let common_buffer_layout = Arc::new(layout_builder.build(device));
        MaterialSystem {
            common_buffer_layout,
            datas: HashMap::default(),
        }
    }

    pub fn get_buffer_layout(&self) -> &wgpu::BindGroupLayout {
        &self.common_buffer_layout
    }

    pub fn get_texture_layout(&self, name: &str) -> Option<&wgpu::BindGroupLayout> {
        self.datas.get(name).map(|v| &v.texture_layout)
    }

    pub fn update(&mut self,world: &mut World,
                  res: &mut RenderResources,
                  commands: &mut CommandEncoder,
                  mat_reader:&mut ManualEventReader<AssetEvent<Material>>) {
        
        let clone_layout = self.common_buffer_layout.clone();
        world.resource_scope(|w, mut materials: Mut<Assets<Material>>| {
            if let Some(events) = w.get_resource::<Events<AssetEvent<Material>>>() {
                for ev in mat_reader.iter(events) {
                    match ev {
                        AssetEvent::Created { handle } => {
                            
                            let mat = materials.get_mut(&handle.id).unwrap();
                            let material_define = self.check_get_material_define(mat,res);
                            let bind_group = material_define.add_material(res, handle.id,&clone_layout);
                            mat.bind_group = Some(bind_group);
                        }
                        AssetEvent::Removed { handle,value } => {
                            let mat = value;
                            if let Some(define) = self.datas.get_mut(&mat.def.name) {
                                define.remove_material(&handle.id)
                            }
                        }
                        _ => {}
                    }
                }


                for define in self.datas.values_mut() {
                    let mut is_buffer_dirty = false;
                    if define.buffer_dirty {
                        is_buffer_dirty = true;
                        let mut group_list = vec![];
                        for idx in 0..define.len {
                            let new_bind_group = define.create_bind_group(idx, res, &clone_layout);
                            group_list.push(Arc::new(new_bind_group));
                        } 
                        for item in define.items.values_mut().chain(define.free_items.iter_mut()) {
                           item.bind_group = group_list[item.index].clone(); 
                        }
                        define.buffer_dirty = false;
                    }

                    let mut has_any_dirty = false;
                    for item in define.items.values_mut() {
                        if let Some(mat) = item.h_material.and_then(|id| materials.get_mut(&id)) {
                            if mat.props.is_dirty() { 
                                item.is_dirty = true; 
                                mat.props.clear_dirty(); 
                            }
                            if mat.is_ready(res) {
                                mat.texture_props.update(res, Some(&define.texture_layout));
                            }
                            if is_buffer_dirty {
                                mat.bind_group = Some(item.bind_group.clone());
                            }
                        }

                        if item.is_dirty {
                            if let Some(mat) = item.h_material.and_then(|id| materials.get_mut(&id)) {
                                if has_any_dirty == false {
                                    res.map_buffer(&define.cache_buffer, wgpu::MapMode::Write);
                                    has_any_dirty = true;
                                }
    
                                if has_any_dirty {
                                    let start = item.index as u64 * define.buffer_item_size;
                                    let buffer = mat.props.get_buffer();
                                    res.write_mapped_buffer(
                                        &define.cache_buffer,
                                        start..(start + buffer.len() as u64),
                                        &mut |bytes, _| {
                                            bytes[0..buffer.len()].copy_from_slice(buffer);
                                        },
                                    );
                                }
                                
                            }
                            item.is_dirty = false;
                        }
                    }
                    if has_any_dirty  {
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
            }
        })
    }

    fn check_get_material_define(&mut self,material:&Material,res:&mut RenderResources) -> &mut MaterialDefine {
        if !self.datas.contains_key(&material.def.name) {
            let new_define = MaterialDefine::new(material.def.clone(),res);
            self.datas.insert(material.def.name.clone(), new_define);
        }
        self.datas.get_mut(&material.def.name).unwrap()
    }
}

impl MaterialDefine {
    pub fn new(define: Arc<MaterialDef>,res:&mut RenderResources) -> Self {
        let buffer_item_size: u64 = align_num_to(define.prop_def.size() as u64, res.device.limits().min_uniform_buffer_offset_alignment.into());
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
            buffer_dirty: false,
        }
    }

    pub fn add_material(&mut self,res: &mut RenderResources,mat_id:HandleId,layout: &wgpu::BindGroupLayout) -> Arc<wgpu::BindGroup> {
        if let Some(mut free_item) = self.free_items.pop() {
            free_item.h_material = Some(mat_id);
            free_item.is_dirty = true;
            let clone_bind_group = free_item.bind_group.clone();
            self.items.insert(mat_id, free_item);
            return clone_bind_group;
        }
        self.len += 1;
        if self.cap < self.len {
            while self.cap < self.len {  self.cap *= 2; }
            let (new_cache_buffer, new_buffer) = Self::alloc_buffer(self.cap, self.buffer_item_size, res);
            self.cache_buffer = new_cache_buffer;
            self.gpu_buffer = new_buffer;
            self.buffer_dirty = true;
            for item in self.free_items.iter_mut().chain(self.items.values_mut()) {
                item.is_dirty = true;
            }
        }
        let index = self.len - 1;
        let bind_group = self.create_bind_group(index, res, layout);
        let item = DefineItem::new(index,mat_id,bind_group);
        let clone_bind_group = item.bind_group.clone();
        self.items.insert(mat_id, item);
        clone_bind_group
    }

    pub fn remove_material(&mut self, id: &HandleId) {
        if let Some(mut rm_item) = self.items.remove(id) {
            rm_item.h_material = None;
            self.free_items.push(rm_item);
        }
    }

    pub fn create_bind_group(&self,item_index:usize,res: &mut RenderResources,layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        let mut build_group_builder = BindGroupBuilder::new();
        let start: u64 = item_index as u64 * self.buffer_item_size;
        build_group_builder.add_buffer_addr(self.gpu_buffer, start, self.buffer_item_size);
        build_group_builder.build(layout, &res.device, res)
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
}


impl DefineItem {
    pub fn new(index:usize,material_id:HandleId,bind_group:wgpu::BindGroup) -> Self {
        DefineItem { 
            index, 
            bind_group:Arc::new(bind_group), 
            h_material: Some(material_id), 
            is_dirty: true 
        }
    }

  
}
