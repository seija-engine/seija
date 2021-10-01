use std::{convert::TryFrom};
use glam::{Mat3, Mat4};
use serde_json::{Map, Value};

#[derive(Debug)]
pub enum UniformType {
    BOOL(Vec<bool>),
   
    FLOAT(Vec<f32>),
    
    INT(Vec<i32>),
    
    UINT(Vec<u32>),
    
    MAT3(Mat3),
    MAT4(Mat4)
} 

#[derive(Debug)]
struct PropInfo {
    name:String,
    typ:UniformType,
    array_size:usize
}
#[derive(Debug)]
pub struct UniformBufferDef {
    props:Vec<PropInfo>
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
        Ok(UniformBufferDef {
            props
        })
    }
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
    if arr_size == 0 {
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
        let size = usize::from_str_radix(size_str,10).unwrap_or(0);
        return (first,size)
    }
    return (first,0)
}

#[test]
fn ttt() {
    let json_string = r#"
      [
          {":name": "name1", ":type": "bool[3]", ":default": [true,false,true] },
          {":name": "f3", ":type": "float[3]", ":default": [7,5,9] },
          {":name": "mat", ":type": "mat4" }
      ]
    "#;
    let v:Value = serde_json::from_str(&json_string).unwrap();
    let ud = UniformBufferDef::try_from(&v).unwrap();
    dbg!(ud);
}