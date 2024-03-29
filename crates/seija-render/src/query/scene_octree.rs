use std::{collections::VecDeque};

use bevy_ecs::prelude::Entity;
use glam::Vec3;
use seija_geometry::{ volume::{AABB3, IAABB},Contains};

#[derive(Debug)]
pub struct SceneElement {
    pub entity:Option<Entity>,
    pub aabb:AABB3
}


pub type NodeId = usize;

#[derive(Debug)]
pub struct SceneOctreeNode {
    pub depth:usize,
    pub aabb:AABB3,
    pub parent:Option<NodeId>,
    pub child_start:Option<NodeId>,
    pub objects:Vec<SceneElement>
}

impl SceneOctreeNode {
    pub fn new(parent:Option<NodeId>,aabb:AABB3,depth:usize) -> Self {
        SceneOctreeNode {parent,child_start:None,objects:vec![],aabb,depth:depth }
    }

    pub fn remove(&mut self,entity:Entity) -> bool {
        if let Some(idx) = self.objects.iter().position(|v| v.entity == Some(entity)) {
            self.objects.remove(idx);
            return true;
        }
        false
    }
}
#[derive(Debug)]
pub struct SceneOctree {
    max_depth:usize,
    root:NodeId,
    pub nodes:Vec<SceneOctreeNode>
}

impl SceneOctree {
    pub fn new(min:Vec3,max:Vec3) -> Self {
        let aabb = AABB3::new(min, max);
        let mut node = SceneOctreeNode::new(None,aabb,0);
        node.child_start = None;
        SceneOctree { max_depth:4,root: 0, nodes: vec![node] }
    }

    pub fn add(&mut self,entity:Entity,aabb:AABB3) -> NodeId {
        loop {
            if let Some(id) = self.node_add(self.root,entity, &aabb) {
                return id
            } else {
                self.grow()
            }
        }
    }

    pub fn remove_bynode(&mut self,id:NodeId,entity:Entity) -> bool {
        if let Some(node) = self.get_node_mut(id) {
           return node.remove(entity);
        }
        false
    }

    //pub fn get_node(&self,id:NodeId) -> Option<&SceneOctreeNode> {
    //    self.nodes.get(id)
    //}

    pub fn get_node_mut(&mut self,id:NodeId) -> Option<&mut SceneOctreeNode> {
        self.nodes.get_mut(id)
    }

    fn grow(&mut self) {
         //grow
         unimplemented!()
    }

    fn node_add(&mut self,id:NodeId,entity:Entity,aabb:&AABB3) -> Option<NodeId> {
        if !self.nodes[id].aabb.contains(aabb) {
            return None;
        }
        let cur_depth = self.nodes[id].depth;
        if  cur_depth >= self.max_depth {
            let elem = SceneElement {entity:Some(entity),aabb:aabb.clone() } ;
            self.nodes[id].objects.push(elem);
            Some(id)
        } else {
            self.check_split(id);
            let offset_index = self.best_fit_child(id, aabb.center());
            let fit_index = self.nodes[id].child_start.unwrap_or(0) + offset_index;
            if let Some(node_id) = self.node_add(fit_index, entity, aabb) {
                Some(node_id)
            } else {
                let elem = SceneElement {entity:Some(entity),aabb:aabb.clone() } ;
                self.nodes[id].objects.push(elem);
                Some(id)
            }
        }
    }

    fn best_fit_child(&self,id:NodeId,center:Vec3) -> usize {
        let self_center = self.nodes[id].aabb.center();
        let x = if center.x <= self_center.x { 0 } else { 1 };
        let y = if center.y >= self_center.y { 0 } else { 4 };
        let z = if center.z <= self_center.z { 0 } else { 2 };
        return x + y + z;
    }

    fn check_split(&mut self,id:NodeId) {
        if self.nodes[id].child_start.is_some() { return ; }
        let cur_depth = self.nodes[id].depth;
        //从上到下从左到右从后到前12个点
        let min = self.nodes[id].aabb.min;
        let max = self.nodes[id].aabb.max;
        let half_y = (max.y + min.y) / 2f32;
        let half_x = (max.x + min.x) / 2f32;
        let half_z = (max.z + min.z) / 2f32;
        let node1 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(min.x, half_y, min.z), Vec3::new(half_x, max.y, half_z)),cur_depth + 1);
        let node2 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(half_x, half_y, min.z), Vec3::new(max.x, max.y, half_z)),cur_depth + 1);
        let node3 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(min.x, half_y, half_z), Vec3::new(half_x, max.y, max.z)),cur_depth + 1);
        let node4 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(half_x, half_y,half_z), Vec3::new(max.x, max.y,  max.z)),cur_depth + 1);
        let node5 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(min.x, min.y, min.z), Vec3::new(half_x,half_y, half_z)),cur_depth + 1);
        let node6 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(half_x, min.y, min.z), Vec3::new(max.x, half_y, half_z)),cur_depth + 1);
        let node7 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(min.x, min.y, half_z), Vec3::new(half_x, half_y, max.z)),cur_depth + 1);
        let node8 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(half_x, min.y,half_z), Vec3::new(max.x, half_y, max.z)),cur_depth + 1);
        self.nodes[id].child_start = Some(self.nodes.len());
        self.nodes.push(node1);
        self.nodes.push(node2);
        self.nodes.push(node3);
        self.nodes.push(node4);
        self.nodes.push(node5);
        self.nodes.push(node6);
        self.nodes.push(node7);
        self.nodes.push(node8);
    }

    
    pub fn update(&mut self,node_id:NodeId,entity:Entity,new_aabb:AABB3) -> Option<NodeId> {
        if let Some(index)  = self.nodes[node_id].objects.iter().position(|v| v.entity == Some(entity)) {
            //没超出当前块
            if  self.nodes[node_id].aabb.contains(&new_aabb) {
                if let Some(new_id) = self.node_add(node_id, entity, &new_aabb) {
                    self.nodes[node_id].objects.remove(index);
                    return Some(new_id);
                } else {
                    self.nodes[node_id].objects[index].aabb = new_aabb;
                    return Some(node_id);
                }
            }
            self.nodes[node_id].objects.remove(index);
            //没超出父节点的块
            if let Some(parent_id) = self.nodes[node_id].parent {
                //用节点尝试添加，如果失败了从最顶层重新添加
                if let Some(new_id) = self.node_add(parent_id, entity, &new_aabb) {
                   return Some(new_id);
                } else {
                   return Some(self.add(entity, new_aabb));
                }
            }
            return Some(self.add(entity, new_aabb));
        }
        None
    }

    pub fn iter_node<'a>(&'a self,id:NodeId) -> impl Iterator<Item = &SceneElement> {
        let mut queue = VecDeque::new();
        queue.push_back(id);
        SceneElementIter {
            tree:self,
            stack:queue,
            cur_object_count:0,
            cur_node:None
        }
    }
}

pub struct SceneElementIter<'a> {
    tree:&'a SceneOctree,
    cur_node:Option<NodeId>,
    stack:VecDeque<NodeId>,
    cur_object_count:usize
}

impl<'a> Iterator for SceneElementIter<'a> {
    type Item = &'a SceneElement;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let node_id = self.try_pop()?;
            let objects = &self.tree.nodes[node_id].objects;
            if self.cur_object_count < objects.len() {
                let ret = &objects[self.cur_object_count];
                self.cur_object_count += 1;
                return Some(ret)
            } else {
                self.cur_object_count = 0;
                self.cur_node = None;
            }
        }
    }
}


impl<'a> SceneElementIter<'a> {
    pub fn try_pop(&mut self) -> Option<NodeId> {
        match self.cur_node {
            Some(node) => Some(node),
            None => {
                let pop_node = self.stack.pop_front()?;
                self.cur_node = Some(pop_node);
                if let Some(start_index) = self.tree.nodes[pop_node].child_start {
                    for idx in 0..8 {
                        self.stack.push_back(start_index + idx);
                    }
                }
                Some(pop_node)
            },
        }
    }
}

#[test]
fn test_tree() {
    let v3_100 = Vec3::new(100f32, 100f32, 100f32);
    let mut scene_tree = SceneOctree::new(-v3_100,v3_100);
    scene_tree.check_split(scene_tree.root);
   
  
    let aabb0 = AABB3::new(Vec3::new(1f32, 1f32, 1f32), Vec3::new(11f32, 11f32, 11f32));
    let e0 = Entity::from_raw(0);

    let add_id = scene_tree.add(e0, aabb0);
    println!("addid:{}",add_id);
    //let b = scene_tree.remove_bynode(add_id, e0);
    //println!("{}",b);

    let new_aabb = AABB3::new(Vec3::new(15f32, 15f32, 15f32), Vec3::new(22f32, 22f32, 22f32));
    let update_id = scene_tree.update(add_id, e0, new_aabb);
    dbg!(update_id);
    //dbg!(&scene_tree);
}

#[test]
fn test_iter() {
    let v3_100 = Vec3::new(100f32, 100f32, 100f32);
    let mut scene_tree = SceneOctree::new(-v3_100,v3_100);
    scene_tree.add(Entity::from_raw(0), AABB3::new(Vec3::new(-10f32, -10f32, -10f32), Vec3::new(10f32, 10f32, 10f32)));
    scene_tree.add(Entity::from_raw(1), AABB3::new(Vec3::new(-40f32, -10f32, -10f32), Vec3::new(-1f32, -1f32, -1f32)));
    scene_tree.add(Entity::from_raw(2), AABB3::new(Vec3::new(10f32, 10f32, 10f32), Vec3::new(20f32, 20f32, 20f32)));

    for elem in scene_tree.iter_node(0) {
        println!("iter elem:{:?}",elem.entity);
    }
}