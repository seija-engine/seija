
#[derive(Debug,PartialEq, Eq)]
pub enum DataType {
    U8,
    U32,
    Bool,
    String,
    Float,
    Custom(String),
    Void
}


#[derive(Debug)]
pub enum Stmt {
   Typedef(DataType,String),
   TypedefStructName(String,String),
   TypedefStruct(String,StructType),
   FuncDefine(FunctionDefine)
}
#[derive(Debug)]
pub struct FunctionDefine {
   pub ret_type:DataType,
   pub name:String
}


#[derive(Debug)]
pub struct StructItem {
    pub typ:DataType,
    pub name:String
}
#[derive(Debug)]
pub struct StructType {
    pub items:Vec<StructItem>,
}
#[derive(Debug)]
pub struct FFIFile {
    pub stmts:Vec<Stmt>   
}