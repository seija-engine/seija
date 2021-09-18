use std::convert::TryFrom;

use serde_json::Value;
use wgpu::MapMode;

pub enum UniformType {
    BOOL(usize,Vec<bool>),
    BOOL2(usize,Vec<bool>),
    BOOL3(usize,Vec<bool>),
    BOOL4(usize,Vec<bool>),
    FLOAT(usize,Vec<f32>),
    FLOAT2(usize,Vec<f32>),
    FLOAT3(usize,Vec<f32>),
    FLOAT4(usize,Vec<f32>),
    INT(usize,Vec<i32>),
    INT2(usize,Vec<i32>),
    INT3(usize,Vec<i32>),
    INT4(usize,Vec<i32>),
    UINT(usize,Vec<u32>),
    UINT2(usize,Vec<u32>),
    UINT3(usize,Vec<u32>),
    UINT4(usize,Vec<u32>),
    MAT3(usize,Vec<f32>),
    MAT4(usize,Vec<f32>)
} 

struct PropInfo {
    name:String,
    typ:UniformType,
    array_size:u32
}

pub struct UniformBufferDef {
    props:Vec<PropInfo>
}

impl TryFrom<&Value> for UniformBufferDef {
    type Error = ();
    fn try_from(value: &Value) -> Result<UniformBufferDef, ()> {
        let arr = value.as_array().ok_or(())?;
        for item in arr {
            if let Some(map) = item.as_object() {
                let prop_type = map.get(":type").and_then(|v| v.as_str()).ok_or(())?;
                let (type_name,arr_size) = split_type_size_str(prop_type);
                let default = map.get(":default");
                match type_name {
                    "bool" | "bool2" | "bool3" | "bool4" => {
                        |s:&Value| { s.as_bool().unwrap_or(false) };
                    },
                    "float" | "float2" | "float3" | "float4" => {
                      //|v:&Value| { v.as_f64().unwrap_or(0) as f32 }
                      todo!()
                    },
                    _ => return Err(())
                };
            }   
        }
        todo!()
    }
}

fn read_prop_default<T>(type_name:&str,arr_size:usize,value:Option<&Value>,read_fn:fn(&Value) -> T) -> Vec<T>  {
    let c = value.unwrap();
    let b = c == 2;
    
    todo!()
}


fn split_type_size_str(type_name:&str) -> (&str,usize) {
    let mut arr = type_name.split('[');
    let first = arr.next().unwrap();
    if let Some(second) = arr.next() {
        let size_str = unsafe { second.get_unchecked(0..second.len() - 1) };
        let size = usize::from_str_radix(size_str,10).unwrap_or(0);
        return (first,size)
    }
    return (first,0)
}

#[test]
fn ttt() {
    let type_name = "float[8]";
    let (t,s) = split_type_size_str(type_name);
    println!("{} {}",t,s);
    
}