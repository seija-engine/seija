use std::fmt::Write;

use crate::{IGenerator, ffi_file::{FFIFile, Stmt, DataTypeFull, DataType}, parse_gen::ParseGenConfig};
use anyhow::{Result};
pub struct CSharpGen;

impl CSharpGen {
    fn write_type(output:&mut String,typ:&DataTypeFull) -> Result<()> {
        if typ.is_ptr {
            if let DataType::Custom(ref s) = typ.typ {
               
                if s.as_str() == "char" {
                    output.write_str("[MarshalAs(UnmanagedType.LPUTF8Str)] string")?;
                    return Ok(());
                }
            }
            output.write_str("IntPtr")?;
            return Ok(());
        }
        match typ.typ {
            DataType::Void  => { output.write_str("void")?; },
            DataType::Bool  => { output.write_str("bool")?; },
            DataType::Float => { output.write_str("float")?; },
            DataType::String => { output.write_str("string")?; },
            DataType::U32 => { output.write_str("uint")?; },
            DataType::U64 => { output.write_str("ulong")?; },
            DataType::Custom(ref c) => { output.write_str(c.as_str())?; },
            _ => {}
        }
        Ok(())
    }

    fn on_process_enums(output:&mut String,ffi_file:&FFIFile) -> Result<()> {
        for stmt in ffi_file.stmts.iter() {
            if let Stmt::EnumDefine(enum_define) = stmt {
                output.write_str("public enum ")?;
                output.write_str(enum_define.name.as_str())?;
                output.write_str(" {\r\n")?;
                for (item_name,value) in enum_define.list.iter() {
                    output.write_str("    ")?;
                    output.write_str(item_name.as_str())?;
                    if let Some(value) = value {
                        output.write_str(" = ")?;
                        output.write_str(value.as_str())?;
                    }
                    output.write_str(",\r\n")?;
                }
                output.write_str(" };\r\n\r\n")?;
            }
        }
        Ok(())
    }

    fn on_process_structs(output:&mut String,ffi_file:&FFIFile) -> Result<()> {
        for stmt in ffi_file.stmts.iter() {
            if let Stmt::TypedefStruct(name,struct_type) = stmt {
                output.write_str("public struct ")?;
                output.write_str(name.as_str())?;
                output.write_str(" {\r\n")?;
                for item in struct_type.items.iter() {
                    output.write_str("    public ")?;
                    CSharpGen::write_type(output, &item.typ)?;
                    output.write_str("  ")?;
                    output.write_str(item.name.as_str())?;
                    output.write_str(";\r\n")?;
                }
                output.write_str("};\r\n\r\n")?;
            }
        }
        Ok(())
    }
}

impl IGenerator for CSharpGen {
    fn on_process(&self,ffi_file:&FFIFile,config:&ParseGenConfig) -> Result<String> {
        let real_class_name = ffi_file.name.replace("-", "_");
        let mut output = String::default();
        output.write_str("using System.Runtime.InteropServices;\r\n")?;
        CSharpGen::on_process_enums(&mut output, ffi_file)?;
        CSharpGen::on_process_structs(&mut output, ffi_file)?;
        output.write_str("public static class ")?;
        output.write_str(real_class_name.as_str())?;
        output.write_str(" {\r\n\r\n")?;
        for stmt in ffi_file.stmts.iter() {
            match stmt {
                Stmt::FuncDefine(func_def) => {
                    output.write_fmt(format_args!("    [DllImport(\"{}\")]\r\n",config.dll_name.as_str()))?;
                    output.write_str("    public static extern ")?;
                    CSharpGen::write_type(&mut output, &func_def.ret_type)?;
                    output.write_str(" ")?;
                    output.write_str(func_def.name.as_str())?;
                    output.write_char('(')?;
                    for idx in 0..func_def.params.len() {
                        let param = &func_def.params[idx];
                        Self::write_type(&mut output, &param.typ)?;
                        output.write_str(" ")?;
                        output.write_str(&param.name)?;
                        if idx < func_def.params.len() - 1 {
                            output.write_char(',')?;
                        }
                    }
                    output.write_str(");\r\n")?;
                    //let func_s = format!("{:?}",func_def);
                    //output.write_str(&func_s)?;
                    output.write_str("\r\n")?;
                },
                _ => {}
            }
        }

        output.write_str("\r\n}")?;
        Ok(output)
    }

    

    
                            
}