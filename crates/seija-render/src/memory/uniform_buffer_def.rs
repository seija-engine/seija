use std::{collections::HashMap, convert::{TryFrom, TryInto}};
use glam::{Mat3, Mat4};
use serde_json::{Value};

#[derive(Debug,Clone)]
pub enum UniformType {
    BOOL(Vec<bool>),
   
    FLOAT(Vec<f32>),
    FLOAT3(Vec<[f32;3]>),
    FLOAT4(Vec<[f32;4]>),
    
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
            UniformType::FLOAT4(_) => 4,
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
            UniformType::FLOAT4(_) => 4,
            UniformType::MAT3(_) => 4,
            UniformType::MAT4(_) => 4,
        }
    }
}

#[derive(Debug)]
pub struct RawUniformInfo {
    pub name:String,
    pub offset:usize,
    pub stride:u8,
    pub size:usize,
    pub typ:UniformType
}

impl RawUniformInfo {
    pub fn get_buffer_offset(&self,idx:usize) -> usize {
        (self.offset as usize + self.stride as usize * idx) * 4
    }
}

#[derive(Debug)]
pub struct ArrayUniformInfo {
    pub name:String,
    pub offset:usize,
    pub stride:usize,
    pub elem_def:UniformBufferDef,
    pub array_size:usize
}

#[derive(Debug)]
pub enum UniformInfo {
    Raw(RawUniformInfo),
    Array(ArrayUniformInfo)
}

impl UniformInfo {
    pub fn get_buffer_offset(&self,idx:usize) -> usize {
        match self {
            UniformInfo::Raw(raw) => {raw.get_buffer_offset(idx)},
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct UniformBufferDef {
    size:usize,
    pub infos:Vec<UniformInfo>,
    names:HashMap<String,usize>
}

impl UniformBufferDef {
    pub fn get_offset(&self,name:&str,index:usize) -> Option<usize> {
        if let Some(idx) = self.names.get(name) {
           let v = self.infos[*idx].get_buffer_offset(index);
           return Some(v)
        }
        None
    }

    pub fn get_array_offset(&self,name:&str,sname:&str,index:usize,sindex:usize) -> Option<usize> {
        if let Some(idx) = self.names.get(name) {
           if let UniformInfo::Array(arr) = &self.infos[*idx] {
               dbg!(&arr.elem_def);
               dbg!(sname);
               if let Some(soffset) = arr.elem_def.get_offset(sname, 0) {
                dbg!(4);
                let offset = (arr.offset + index * arr.stride) * 4;
                dbg!(soffset + offset);
                return Some(soffset + offset)
              }
           }
         }

         None
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub struct RawPropInfo {
    pub name:String,
    pub typ:UniformType,
    pub array_size:usize
}
#[derive(Debug)]
pub struct ArrayPropInfo {
    pub name:String,
    pub props:Vec<RawPropInfo>,
    pub array_size:usize
}

#[derive(Debug)]
pub enum PropInfo {
    Raw(RawPropInfo),
    Array(ArrayPropInfo)
}

#[derive(Debug)]
pub struct PropInfoList(pub Vec<PropInfo>);

impl TryFrom<&Value> for PropInfoList {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let arr = value.as_array().ok_or(())?;
        let lst:Vec<PropInfo> = arr.iter().filter_map(|v| {
            match read_prop(v) {
                Ok(v) => v,
                Err(_) => {
                    log::error!("read prop error:{:?}",&v);
                    None
                }
            }
        }).collect();
        Ok(PropInfoList(lst))
    }
}


struct BuildPropContext {
    n3:u32,
    n83:u8,
    name_index:usize,
    offset:u32,
    name_map:HashMap<String,usize>
}

impl Default for BuildPropContext {
    fn default() -> BuildPropContext {
        BuildPropContext { n3: !3, n83: !3, name_index:0,offset:0,name_map:HashMap::default() }
    }
}

fn  build_props(props:&Vec<PropInfo>) -> UniformBufferDef {
    let mut info_list:Vec<UniformInfo> = Vec::new();
    let mut prop_ctx = BuildPropContext::default();
    for prop in props {
        match prop  {
            PropInfo::Raw(raw) => {
                let raw_info = build_prop_raw(&mut prop_ctx, raw);
                info_list.push(UniformInfo::Raw(raw_info));
            },
            PropInfo::Array(arr) => {
                let mut arr_infos:Vec<UniformInfo> = Vec::new();
                let mut elem_ctx = BuildPropContext::default();
                for raw_prop in arr.props.iter() {
                    let raw_info = build_prop_raw(&mut elem_ctx, &raw_prop);
                    arr_infos.push(UniformInfo::Raw(raw_info));
                }
                let struct_size:u32 = 4 * ((elem_ctx.offset + 3) & elem_ctx.n3);
                let def = UniformBufferDef {
                    size:struct_size as usize,
                    infos:arr_infos,
                    names:elem_ctx.name_map
                };
                let align:u32   =  7 & prop_ctx.n3;
                let stride:u32  = (struct_size + 3) & prop_ctx.n3;
                let padding:u32 = (align - (prop_ctx.offset % align)) % align;
                prop_ctx.offset += padding;
                let arr_def = ArrayUniformInfo {
                    offset:prop_ctx.offset as usize,
                    stride:stride as usize,
                    name:arr.name.to_string(),
                    array_size:arr.array_size,
                    elem_def:def
                };
                info_list.push(UniformInfo::Array(arr_def));
                prop_ctx.name_map.insert(arr.name.to_string(), prop_ctx.name_index);
                prop_ctx.offset += (stride as u32) * (arr.array_size as u32);
                prop_ctx.name_index += 1;
            }
        }
    }

    let size:u32 = 4 * ((prop_ctx.offset + 3) & prop_ctx.n3);
    UniformBufferDef {
        size:size as usize,
        infos:info_list,
        names:prop_ctx.name_map
    }
}

fn build_prop_raw(ctx:&mut BuildPropContext,prop:&RawPropInfo) -> RawUniformInfo {
    let mut align = prop.typ.base_align();
    let mut stride = prop.typ.stride();
    if prop.array_size > 1 {
        align = (align + 3) & ctx.n3;
        stride = (stride + 3) & ctx.n83;
    }
    let padding:u32 = (align - (ctx.offset % align)) % align;
    ctx.offset += padding;

    let info = RawUniformInfo {
        name:prop.name.clone(),
        typ:prop.typ.clone(),
        offset:ctx.offset as usize,
        stride,
        size:prop.array_size
    };
    ctx.name_map.insert(prop.name.to_string(), ctx.name_index);
    ctx.offset += (stride as u32) * (prop.array_size as u32);
    ctx.name_index += 1;
    info
}



fn read_prop(value:&Value) -> Result<Option<PropInfo>,()>   {
    let map = value.as_object().ok_or(())?;
    let name = map.get(":name").and_then(Value::as_str).ok_or(())?;
    let json_type = map.get(":type").ok_or(())?;
    match json_type {
        Value::String(str)   => read_prop_str(name,str,map).map(|v| v.map(PropInfo::Raw)),
        Value::Array(arr) => read_prop_array(name, arr, map).map(|v| v.map(PropInfo::Array)),
        _ => Ok(None)
    }
}

fn read_prop_array(name:&str,arr:&Vec<Value>,map:&serde_json::Map<String,Value>) -> Result<Option<ArrayPropInfo>,()> {
    let mut type_arr:Vec<RawPropInfo> = vec![];
    for json_item in arr.iter() {
        let map = json_item.as_object().ok_or(())?;
        let type_str = json_item.get(":type").and_then(Value::as_str).ok_or(())?;
        let name = map.get(":name").and_then(Value::as_str).ok_or(())?;
        if let Some(item)  = read_prop_str(name, type_str, map)? {
            type_arr.push(item);
        }
    }
    let size = map.get(":size").and_then(Value::as_i64).ok_or(())?; 
    Ok(Some(ArrayPropInfo {
        name:name.to_string(),
        props:type_arr,
        array_size:size as usize
    }))
}

fn read_prop_str(name:&str,type_str:&str,map:&serde_json::Map<String,Value>) -> Result<Option<RawPropInfo>,()> {
    let (type_name,array_size) = split_type_size_str(type_str);
    let default = map.get(":default");
    match type_name {
        "bool" => {
            let f = |v:&Value| { v.as_bool().unwrap_or(false)};
            let arr = read_default(default,array_size,f,false)?;
            return Ok(Some(RawPropInfo { name:name.to_string(), array_size, typ:UniformType::BOOL(arr) }))
         },
         "float" => {
             let f = |v:&Value| { v.as_f64().unwrap_or(0f64) as f32  };
             let arr = read_default(default,array_size,f,0f32)?;
             return Ok(Some(RawPropInfo { name:name.to_string(), array_size, typ:UniformType::FLOAT(arr) }))
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
            return Ok(Some(RawPropInfo { name:name.to_string(), array_size, typ:UniformType::FLOAT3(arr) }))
        },
        "float4" => {
            let f = |v:&Value| { 
                let arr = v.as_array().unwrap();
                let x = arr[0].as_f64().unwrap() as f32;
                let y = arr[1].as_f64().unwrap() as f32;
                let z = arr[2].as_f64().unwrap() as f32;
                let w = arr[3].as_f64().unwrap() as f32;
                [x,y,z,w]
            };
            let arr:Vec<[f32;4]> = read_default::<[f32;4]>(default,array_size,f,[0f32,0f32,0f32,0f32])?;
            return Ok(Some(RawPropInfo { name:name.to_string(), array_size, typ:UniformType::FLOAT4(arr) }))
        },
         "int" => {
            let f = |v:&Value| { v.as_i64().unwrap_or(0i64) as i32  };
            let arr = read_default(default,array_size,f,0i32)?;
            return Ok(Some(RawPropInfo { name:name.to_string(), array_size, typ:UniformType::INT(arr) }))
         },
         "uint" => {
            let f = |v:&Value| { v.as_i64().unwrap_or(0i64) as u32  };
            let arr = read_default(default,array_size,f,0u32)?;
            return Ok(Some(RawPropInfo { name:name.to_string(), array_size, typ:UniformType::UINT(arr) }))
         },
         "mat3" => {
             let def = Mat3::default();
             return Ok(Some(RawPropInfo { name:name.to_string(), array_size, typ:UniformType::MAT3(def) }))
         },
         "mat4" => {
            return Ok(Some(RawPropInfo { name:name.to_string(), array_size, typ:UniformType::MAT4(Mat4::default()) }))
         },
         _ => return Ok(None)
    }
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


impl TryFrom<&Value> for UniformBufferDef {
    type Error = ();
    fn try_from(value: &Value) -> Result<UniformBufferDef, ()> {
        let prop_list:PropInfoList = value.try_into()?; 
        UniformBufferDef::try_from(&prop_list)
    }
}

impl TryFrom<&PropInfoList> for UniformBufferDef {
    type Error = ();
    fn try_from(prop_list: &PropInfoList) -> Result<UniformBufferDef, ()> {
        Ok( build_props(&prop_list.0))
    }
}

#[test]
fn ttt() {
    let json_string = r#"
      [
          {":name": "f",  ":type": "float", ":default": 0 },
          {":name": "farr", ":type":[{":name": "inner", ":type":"float3",":default":[111,111,111]  }],":size": 3 },
          {":name": "f2", ":type":"float3"  }
      ]
    "#;
    let v:Value = serde_json::from_str(&json_string).unwrap();
    let def = UniformBufferDef::try_from(&v).unwrap();
    dbg!(&def);
}
