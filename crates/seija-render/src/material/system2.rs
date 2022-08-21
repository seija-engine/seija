use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use super::{Material, MaterialDef, MaterialDefineAsset};
use crate::{
    memory::align_num_to,
    pipeline::render_bindings::{BindGroupBuilder, BindGroupLayoutBuilder},
    resource::{BufferId, RenderResources},
};
use bevy_ecs::{
    change_detection::Mut,
    entity::Entity,
    event::{Events, ManualEventReader},
    prelude::{World},
};
use seija_asset::{AssetEvent, Assets, Handle, HandleId};
use smol_str::SmolStr;
use wgpu::CommandEncoder;

pub struct MaterialSystem {
    define_reader: ManualEventReader<AssetEvent<MaterialDefineAsset>>,
    common_buffer_layout: wgpu::BindGroupLayout,
    datas: HashMap<SmolStr, MaterialDefine>,
}

impl MaterialSystem {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut layout_builder = BindGroupLayoutBuilder::new();
        layout_builder.add_uniform(wgpu::ShaderStage::VERTEX_FRAGMENT);
        let common_buffer_layout = layout_builder.build(device);
        MaterialSystem {
            define_reader: Default::default(),
            common_buffer_layout,
            datas: HashMap::default(),
        }
    }

    pub fn get_texture_layout(&self, name: &str) -> Option<&wgpu::BindGroupLayout> {
        self.datas.get(name).map(|v| &v.texture_layout)
    }

    fn add_material_define(&mut self, define: Arc<MaterialDef>, res: &mut RenderResources) {
        self.datas
            .insert(define.name.clone(), MaterialDefine::new(define, res));
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
            let rm_list: Vec<Entity> = world.removed::<Handle<Material>>().collect();
            for define in self.datas.values_mut() {
                for rm_entity in rm_list.iter() {
                    define.remove_material(rm_entity.id())
                }
            }
        };
        //update define
        if let Some(define_events) = world.get_resource::<Events<AssetEvent<MaterialDefineAsset>>>()
        {
            let mut frame_add_defines: HashSet<Handle<MaterialDefineAsset>> = HashSet::default();
            for define_event in self.define_reader.iter(define_events) {
                match define_event {
                    AssetEvent::Created { handle } => {
                        frame_add_defines.insert(handle.clone_weak());
                    }
                    AssetEvent::Removed { handle } => {
                        frame_add_defines.remove(handle);
                    }
                    _ => {}
                }
            }
            if let Some(assets) = world.get_resource::<Assets<MaterialDefineAsset>>() {
                for define in frame_add_defines.iter() {
                    if let Some(def_asset) = assets.get(&define.id) {
                        self.add_material_define(def_asset.define.clone(), res);
                    }
                }
            }
        }

        let mut query = world.query::<(Entity, &Handle<Material>)>();
        world.resource_scope(|w, mut materials: Mut<Assets<Material>>| {
            //update material
            for (e, h_mat) in query.iter(w) {
                let h_mat: &Handle<Material> = h_mat;
                if let Some(mat) = materials.get_mut(&h_mat.id) {
                    if let Some(define) = self.datas.get_mut(mat.def.name.as_str()) {
                        if mat.is_ready(res) {
                            mat.texture_props.update(res, Some(&define.texture_layout));
                        }

                        if !define.items.contains_key(&e.id()) {
                            define.add_material(e.id(), res);
                        }

                        let did = if mat.props.is_dirty() {
                            Some(h_mat.id.clone())
                        } else {
                            None
                        };
                        define.update_buffer(e.id(), res, &self.common_buffer_layout, mat, did);
                    }
                }
            }

            for define in self.datas.values_mut() {
                let mut cur_has_dirty = false;
                for (_, item) in define.items.iter_mut() {
                    
                    if let Some(dirty_id) = item.dirty_hid.as_ref() {
                        if let Some(mat) = materials.get(dirty_id) {
                            if cur_has_dirty == false {
                                res.map_buffer(&define.cache_buffer, wgpu::MapMode::Write);
                                log::error!("map!!!!!!!!!!");
                                cur_has_dirty = true;
                            }

                            if cur_has_dirty {
                                let start = item.index as u64 * define.buffer_item_size;
                                let buffer = mat.props.get_buffer();
                                log::error!("write");
                                res.write_mapped_buffer(
                                    &define.cache_buffer,
                                    start..(start + buffer.len() as u64),
                                    &mut |bytes, _| {
                                        bytes[0..buffer.len()].copy_from_slice(buffer);
                                    },
                                );
                            }
                            
                        }
                        item.dirty_hid = None;
                    }
                    
                }
                if cur_has_dirty {
                    log::error!("unmap!!!!!!!!!!");
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
struct DefineItem {
    index: usize,
    dirty_hid: Option<HandleId>,
}

impl DefineItem {
    pub fn new(index: usize) -> Self {
        DefineItem {
            index,
            dirty_hid: None,
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

    items: fnv::FnvHashMap<u32, DefineItem>,
    free_items: Vec<DefineItem>,

    buffer_dirty: bool,
}

impl MaterialDefine {
    pub fn new(define: Arc<MaterialDef>, res: &mut RenderResources) -> Self {
        let buffer_item_size: u64 =
            align_num_to(define.prop_def.size() as u64, wgpu::BIND_BUFFER_ALIGNMENT);
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
            usage: wgpu::BufferUsage::COPY_SRC | wgpu::BufferUsage::MAP_WRITE,
            mapped_at_creation: false,
        });

        let uniform_buffer = res.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: cap as u64 * item_size as u64,
            usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
            mapped_at_creation: false,
        });

        (cache_buffer, uniform_buffer)
    }

    pub fn add_material(&mut self, id: u32, res: &mut RenderResources) {
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
            let (cache_buffer, buffer) = Self::alloc_buffer(self.cap, self.buffer_item_size, res);
            self.cache_buffer = cache_buffer;
            self.gpu_buffer = buffer;
            self.buffer_dirty = true;
        }

        let index = self.len - 1;

        self.items.insert(id, DefineItem::new(index));
    }

    pub fn remove_material(&mut self, id: u32) {
        if let Some(rm_item) = self.items.remove(&id) {
            self.free_items.push(rm_item);
        }
    }

    pub fn update_buffer(
        &mut self,
        id: u32,
        res: &mut RenderResources,
        layout: &wgpu::BindGroupLayout,
        material: &mut Material,
        dirty_hid: Option<HandleId>,
    ) {
        if self.buffer_dirty || material.bind_group.is_none() {
            if let Some(item) = self.items.get_mut(&id) {
                let mut build_group_builder = BindGroupBuilder::new();
                let start: u64 = item.index as u64 * self.buffer_item_size;
                build_group_builder.add_buffer_addr(self.gpu_buffer, start, self.buffer_item_size);
                material.bind_group = Some(build_group_builder.build(layout, &res.device, res));
                item.dirty_hid = dirty_hid;
            }
        }
    }
}
