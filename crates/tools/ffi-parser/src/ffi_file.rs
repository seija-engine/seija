
#[derive(Debug,PartialEq, Eq)]
pub enum DataType {
    U8,
    U32,
    U64,
    Bool,
    String,
    Float,
    Custom(String),
    Void
}

#[derive(Debug)]
pub struct DataTypeFull {
    pub typ:DataType,
    pub is_ptr:bool
}


#[derive(Debug)]
pub enum Stmt {
   Typedef(DataType,String),
   TypedefStructName(String,String),
   TypedefStruct(String,StructType),
   FuncDefine(FunctionDefine),
   EnumDefine(EnumDefine)
}
#[derive(Debug)]
pub struct FunctionDefine {
   pub ret_type:DataTypeFull,
   pub name:String,
   pub params:Vec<FuncParam>
}
#[derive(Debug)]
pub struct FuncParam {
    pub name:String,
    pub typ:DataTypeFull
}

#[derive(Debug)]
pub struct StructItem {
    pub typ:DataTypeFull,
    pub name:String
}
#[derive(Debug)]
pub struct StructType {
    pub items:Vec<StructItem>,
}

#[derive(Debug)]
pub struct EnumDefine {
   pub name:String,
   pub list:Vec<(String,Option<String>)>
}

#[derive(Debug)]
pub struct FFIFile {
    pub name:String,
    pub stmts:Vec<Stmt>   
}