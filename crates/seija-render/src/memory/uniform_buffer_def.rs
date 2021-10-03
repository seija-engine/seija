use std::{collections::HashMap, convert::TryFrom};
use glam::{Mat3, Mat4};
use serde_json::{Map, Value};

#[derive(Debug,Clone)]
pub enum UniformType {
    BOOL(Vec<bool>),
   
    FLOAT(Vec<f32>),
    FLOAT3(Vec<[f32;3]>),
    
    INT(Vec<i32>),
    
    UINT(Vec<u32>),
    
    MAT3(Mat3),
    MAT4(Mat4)
}

impl UniformType {
    pub fn stride(&self) -> u8 {
        match self {
            UniformType::BOOL(_) => 1,
            UniformType::FLOAT(_) => 1,
            UniformType::INT(_) => 1,
            UniformType::UINT(_) => 1,
            UniformType::FLOAT3(_) => 3,
            UniformType::MAT3(_) => 12,
            UniformType::MAT4(_) => 16,
        }
    }

    pub fn base_align(&self) -> u32 {
        match self {
            UniformType::BOOL(_) => 1,
            UniformType::FLOAT(_) => 1,
            UniformType::INT(_) => 1,
            UniformType::UINT(_) => 1,
            UniformType::FLOAT3(_) => 4,
            UniformType::MAT3(_) => 4,
            UniformType::MAT4(_) => 4,
        }
    }
}



#[derive(Debug)]
pub struct UniformInfo {
    pub name:String,
    pub offset:usize,
    pub stride:u8,
    pub size:usize,
    pub typ:UniformType
}

impl UniformInfo {
    pub fn get_buffer_offset(&self,idx:usize) -> usize {
        (self.offset as usize + self.stride as usize * idx) * 4
    }
}

#[derive(Debug)]
pub struct UniformBufferDef {
    size:usize,
    pub infos:Vec<UniformInfo>,
    names:HashMap<String,usize>,
}

impl UniformBufferDef {
    pub fn get_offset(&self,name:&str,index:usize) -> Option<usize> {
        if let Some(idx) = self.names.get(name) {
           let v = self.infos[*idx].get_buffer_offset(index);
           return Some(v)
        }
        None
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
struct PropInfo {
    pub name:String,
    pub typ:UniformType,
    pub array_size:usize
}

impl TryFrom<&Value> for UniformBufferDef {
    type Error = ();
    fn try_from(value: &Value) -> Result<UniformBufferDef, ()> {
        let mut props:Vec<PropInfo> = Vec::new();
        let arr = value.as_array().ok_or(())?;
        for item in arr {
            if let Some(map) = item.as_object() {
                let prop_name = map.get(":name").and_then(|v| v.as_str()).ok_or(())?;
                let prop = read_prop(prop_name,map)?;
                props.push(prop);
            }   
        }
       
        let (infos,name_map,size) = build_props(&props);        
        Ok(UniformBufferDef {
            size,
            infos,
            names:name_map
        })
    }
}

fn build_props(props:&Vec<PropInfo>) -> (Vec<UniformInfo>,HashMap<String,usize>,usize) {
    let mut info_list:Vec<UniformInfo> = Vec::new();
    let mut name_map:HashMap<String,usize> = HashMap::new();
    let mut offset:u32 = 0;
    let n3 = !3u32;
    let n83 = !3u8;
    let mut name_index:usize = 0;
    for prop in props {
       let mut align = prop.typ.base_align();
       let mut stride = prop.typ.stride();
       if prop.array_size > 1 {
           align = (align + 3) & n3;
           stride = (stride + 3) & n83;
       }
       let padding:u32 = (align - (offset % align)) % align;
       offset += padding;
       let info = UniformInfo {
           name:prop.name.clone(),
           typ:prop.typ.clone(),
           offset:offset as usize,
           stride,
           size:prop.array_size
       };
       info_list.push(info);
       offset += (stride as u32) * (prop.array_size as u32);
       name_map.insert(prop.name.to_string(), name_index);
       name_index += 1;
    }
    let size:u32 = 4 * ((offset + 3) & n3);
    (info_list,name_map,size as usize)
}

fn read_prop(name:&str,map:&Map<String,Value>) -> Result<PropInfo,()>   {
    let prop_type = map.get(":type").and_then(|v| v.as_str()).ok_or(())?;
    let (type_name,array_size) = split_type_size_str(prop_type);
    let default = map.get(":default");
    match type_name {
        "bool" => {
            let f = |v:&Value| { v.as_bool().unwrap_or(false)};
            let arr = read_default(default,array_size,f,false)?;
            return Ok(PropInfo { name:name.to_string(), array_size, typ:UniformType::BOOL(arr) })
         },
         "float" => {
             let f = |v:&Value| { v.as_f64().unwrap_or(0f64) as f32  };
             let arr = read_default(default,array_size,f,0f32)?;
             return Ok(PropInfo { name:name.to_string(), array_size, typ:UniformType::FLOAT(arr) })
         },
         "float3" => {
            let f = |v:&Value| { 
                let arr = v.as_array().unwrap();
                let x = arr[0].as_f64().unwrap() as f32;
                let y = arr[1].as_f64().unwrap() as f32;
                let z = arr[2].as_f64().unwrap() as f32;
                [x,y,z]
            };
            let arr:Vec<[f32;3]> = read_default::<[f32;3]>(default,array_size,f,[0f32,0f32,0f32])?;
            return Ok(PropInfo { name:name.to_string(), array_size, typ:UniformType::FLOAT3(arr) })
        },
         "int" => {
            let f = |v:&Value| { v.as_i64().unwrap_or(0i64) as i32  };
            let arr = read_default(default,array_size,f,0i32)?;
            return Ok(PropInfo { name:name.to_string(), array_size, typ:UniformType::INT(arr) })
         },
         "uint" => {
            let f = |v:&Value| { v.as_i64().unwrap_or(0i64) as u32  };
            let arr = read_default(default,array_size,f,0u32)?;
            return Ok(PropInfo { name:name.to_string(), array_size, typ:UniformType::UINT(arr) })
         },
         "mat3" => {
             let def = Mat3::default();
             return Ok(PropInfo { name:name.to_string(), array_size, typ:UniformType::MAT3(def) })
         },
         "mat4" => {
            return Ok(PropInfo { name:name.to_string(), array_size, typ:UniformType::MAT4(Mat4::default()) })
         },
         _ => {}
    }
    Err(())
}

fn read_default<T:Copy>(default:Option<&Value>,arr_size:usize,f:fn(&Value) -> T,def:T) -> Result<Vec<T>,()>  {
    if arr_size == 1 {
        let def_val = match default {
            Some(v) => f(v),
            None => def
        };
        Ok(vec![def_val])
    } else {
        let arr:Vec<T> = match default {
            Some(v) => {
                let arr = v.as_array().ok_or(())?;
                arr.iter().map(|v| f(v)).collect()
            },
            None => { (0..arr_size).map(|_| def).collect() }
        };
        Ok(arr)
    }
}


fn split_type_size_str(type_name:&str) -> (&str,usize) {
    let mut arr = type_name.split('[');
    let first = arr.next().unwrap();
    if let Some(second) = arr.next() {
        let size_str = unsafe { second.get_unchecked(0..second.len() - 1) };
        let size = usize::from_str_radix(size_str,10).unwrap_or(1);
        return (first,size)
    }
    return (first,1)
}

#[test]
fn ttt() {
    let json_string = r#"
      [
          {":name": "name1", ":type": "bool[3]", ":default": [true,false,true] },
          {":name": "f", ":type": "float", ":default": 0 },
          {":name": "f3", ":type": "float[3]", ":default": [7,5,9] },
          {":name": "mat", ":type": "mat4" }
      ]
    "#;
    let v:Value = serde_json::from_str(&json_string).unwrap();
    let def = UniformBufferDef::try_from(&v).unwrap();
    let f3_0 = def.get_offset("f3", 0);
    dbg!(f3_0);
    dbg!(def.size());
  
}
