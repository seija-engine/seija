use bevy_ecs::prelude::Entity;
use glam::Vec3;
use seija_geometry::{octree::{Octree, OctantId}, volume::{AABB3, IAABB},Contains};

#[derive(Debug)]
pub struct SceneElement {
    entity:Option<Entity>,
    aabb:AABB3
}


type NodeId = usize;

#[derive(Debug)]
pub struct SceneOctreeNode {
    pub aabb:AABB3,
    parent:Option<NodeId>,
    pub children:Vec<NodeId>,
    pub objects:Vec<SceneElement>
}

impl SceneOctreeNode {
    pub fn new(parent:Option<NodeId>,aabb:AABB3) -> Self {
        SceneOctreeNode {parent,children:vec![],objects:vec![],aabb }
    }

   
}
#[derive(Debug)]
pub struct SceneOctree {
    root:NodeId,
    nodes:Vec<SceneOctreeNode>
}

impl SceneOctree {
    pub fn new(min:Vec3,max:Vec3) -> Self {
        let aabb = AABB3::new(min, max);
        let node = SceneOctreeNode::new(None,aabb);
        SceneOctree { root: 0, nodes: vec![node] }
    }

    pub fn add(&mut self,entity:Entity,aabb:AABB3) {
        if self.node_add(self.root,entity, aabb) {

            //grow
            todo!()
        }
    }

    fn node_add(&mut self,id:NodeId,entity:Entity,aabb:AABB3) -> bool {
        if !self.nodes[id].aabb.contains(&aabb) {
            return false;
        }
        self.check_split(id);
        let offset_index = self.best_fit_child(id, aabb.center());
        let fit_index = self.nodes[id].children[0] + offset_index;
        if self.nodes[fit_index].aabb.contains(&aabb) {

        } else {
            todo!()
        }
        true
    }

    fn best_fit_child(&self,id:NodeId,center:Vec3) -> usize {
        let self_center = self.nodes[id].aabb.center();
        let x = if center.x <= self_center.x { 0 } else { 1 };
        let y = if center.y >= self_center.y { 0 } else { 4 };
        let z = if center.z <= self_center.z { 0 } else { 2 };
        return x + y + z;
    }

    fn check_split(&mut self,id:NodeId) {
        if self.nodes[id].children.len() > 0 { return ; }
        //从上到下从左到右从后到前12个点
        let min = self.nodes[id].aabb.min;
        let max = self.nodes[id].aabb.max;
        let half_y = (max.y + min.y) / 2f32;
        let half_x = (max.x + min.x) / 2f32;
        let half_z = (max.z + min.z) / 2f32;
        let node1 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(min.x, half_y, min.z), Vec3::new(half_x, max.y, half_z)));
        let node2 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(half_x, half_y, min.z), Vec3::new(max.x, max.y, half_z)));
        let node3 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(min.x, half_y, half_z), Vec3::new(half_x, max.y, max.z)));
        let node4 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(half_x, half_y,half_z), Vec3::new(max.x, max.y,  max.z)));
        let node5 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(min.x, min.y, min.z), Vec3::new(half_x,half_y, half_z)));
        let node6 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(half_x, min.y, min.z), Vec3::new(max.x, half_y, half_z)));
        let node7 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(min.x, min.y, half_z), Vec3::new(half_x, half_y, max.z)));
        let node8 = SceneOctreeNode::new(Some(id), AABB3::new(Vec3::new(half_x, min.y,half_z), Vec3::new(max.x, half_y, max.z)));
        let start_index = self.nodes.len();
        self.nodes.push(node1);
        self.nodes.push(node2);
        self.nodes.push(node3);
        self.nodes.push(node4);
        self.nodes.push(node5);
        self.nodes.push(node6);
        self.nodes.push(node7);
        self.nodes.push(node8);
        for idx in start_index..(start_index + 8) {
            self.nodes[id].children.push(idx);
        }
    }

   
}

#[test]
fn test_tree() {
    let v3_100 = Vec3::new(100f32, 100f32, 100f32);
    let mut scene_tree = SceneOctree::new(-v3_100,v3_100);
    scene_tree.check_split(scene_tree.root);
    dbg!(scene_tree);
   
}