use std::collections::VecDeque;

use crate::{lex_string::LexString, ffi_file::FFIFile};
use crate::ffi_file::{Stmt,DataType,StructType, StructItem, FunctionDefine, EnumDefine, FuncParam, DataTypeFull};
#[derive(Debug)]
pub enum ParseError {
    EOF,
    ErrType(Token),
    ErrToken(Token),
    //ErrFnName,
    ErrEnumFormat
}

#[derive(Debug,PartialEq, Eq)]
pub enum Token {
    Type(DataType),
    Symbol(String),
    //Keyword(String),
    Typedef,
    Struct,
    Enum,
    LeftBrace, // {}
    RightBrace,
    LeftParent,//(
    RightParent,
    Comma,
    Eq,
    Const
}

impl Token {
    pub fn cast_sym(self) -> Result<String,ParseError> {
        match self {
            Token::Symbol(sym) => Ok(sym),
            _ => {
                Err(ParseError::ErrToken(self))
            }
        }
    }
}




pub struct FFIFileParser<'a> {
    name:String,
    cache_tokens:VecDeque<Token>,
    lex_string:LexString<'a>
}

impl<'a> FFIFileParser<'a> {
    pub fn new(file_source:&'a str,name:String) -> Self {
        let lexer = LexString::new(file_source, 5);
        FFIFileParser { name, cache_tokens:Default::default(),lex_string:lexer }
    }

    pub fn parse(&mut self) -> Result<FFIFile,ParseError> {
        let mut stmt_list:Vec<Stmt> = vec![];
        
        loop {
            match self.parse_item() {
                Ok(stmt) => {
                    stmt_list.push(stmt);
                },
                Err(ParseError::EOF) => {
                    break; 
                }
                Err(err) => {
                    eprintln!("parse err:{:?}",err);
                }
            }
        }

        Ok(FFIFile {name:self.name.clone(), stmts:stmt_list} )
    }

    fn parse_item(&mut self) -> Result<Stmt,ParseError> {
       let tok = self.next_token(false)?;
       
       match tok {
        Token::Typedef => {
            let next = self.next_token(false)?;
           
            if next == Token::Struct {
                let new_name = self.next_token(true)?.cast_sym()?;
                let next_tok = self.next_token(true)?;
                if next_tok == Token::LeftBrace {
                    let struct_type = self.parse_struct()?;
                    return Ok(Stmt::TypedefStruct(new_name,struct_type));
                } else {
                   return Ok(Stmt::TypedefStructName(new_name,next_tok.cast_sym()?));
                }
            } else if let Token::Type(typ) = next {
                
                let next_tok = self.next_token(true)?;
                if next_tok == Token::LeftParent {
                   let func_def = self.parse_fn_ptr(typ)?;
                   return Ok(Stmt::TypeDefFuncPtr(func_def) );
                } else {
                    return Ok(Stmt::Typedef(typ,next_tok.cast_sym()?))
                }
            } else {
                
                return Err(ParseError::ErrToken(next));
            }
        },
        Token::Enum => {
            let enum_define = self.parse_enum()?;
            
            return Ok(Stmt::EnumDefine(enum_define));
        }
         tok => {
           
            let func_define = self.parse_func_define(tok)?;
            return Ok(Stmt::FuncDefine(func_define))
         }
       }
    }

    fn parse_struct(&mut self) -> Result<StructType,ParseError> {
        let mut items:Vec<StructItem> = vec![];
        loop {
            let mut item_type = self.next_token(false)?;
            if item_type == Token::RightBrace {
                break;
            };
            if item_type == Token::Struct {
                item_type = self.next_token(false)?;
            }
            let real_type = match item_type {
                Token::Type(typ) => { typ },
                Token::Symbol(sym) => {DataType::Custom(sym) },
                tok => { return Err(ParseError::ErrType(tok)); }
            };
            let full_type = self.fill_data_type(real_type);
            

            let item_mame = self.next_token(true)?.cast_sym()?;
            let item = StructItem { typ:full_type,name:item_mame };
            items.push(item);
        }
        if let Some(_) = self.lex_string.take_while(|chr| chr== ';') {
            self.lex_string.next();
        }
        let struct_type = StructType { items };
        Ok(struct_type)
    }

    fn fill_data_type(&mut self,typ:DataType) -> DataTypeFull {
        self.skip_white();
        let mut is_ptr = false;
        if self.lex_string.lookahead(1) == Some('*') {
            self.lex_string.next();
            is_ptr = true;
        };

        DataTypeFull { typ, is_ptr }
    }

    fn parse_enum(&mut self) -> Result<EnumDefine,ParseError> {
        let enum_name = self.next_token(true)?.cast_sym()?;
        
        if self.next_token(false)? != Token::LeftBrace {
            return Err(ParseError::ErrEnumFormat);
        }
        let mut list = vec![];
        loop {
          
           let fst_tok = self.next_token(true)?;
           if fst_tok == Token::RightBrace { break; }
           let field_name = fst_tok.cast_sym()?;
           let snd_tok = self.next_token(false)?;
           match snd_tok {
                Token::Eq => {
                    let value_sym = self.next_token(true)?.cast_sym()?;
                    list.push((field_name,Some(value_sym)));
                    let trd_tok = self.next_token(false)?;
                    println!("trd_tok:{:?}",&trd_tok);
                    match trd_tok {
                        Token::Comma => {},
                        _ => { break; }
                    }
                },
                Token::Comma => {
                    list.push((field_name,None));
                },
                Token::RightBrace => { break; },
                other => {
                    println!("other:{:?}",other);
                    return Err(ParseError::ErrEnumFormat ); 
                }
           }
        }
       
      
        Ok(EnumDefine { name: enum_name, list })
    }

    fn parse_fn_ptr(&mut self,tok_typ:DataType) -> Result<FunctionDefine,ParseError> {
        self.lex_string.next(); // skip *
        let name = self.next_token(true)?.cast_sym()?;
        self.next_token(false)?; // skip )
       
        self.next_token(false)?; // skip (
        let params = self.parse_params()?;
        Ok( FunctionDefine {
             ret_type:DataTypeFull {typ:tok_typ,is_ptr:false},
             name,
             params
        })
    }

    fn parse_params(&mut self) -> Result<Vec<FuncParam>,ParseError> {
        let mut params:Vec<FuncParam> = vec![];
        loop {
            let mut type_name = self.next_token(false)?;
            if type_name == Token::RightParent {
                break;
            }
            if type_name == Token::Type(DataType::Void) {
                continue;
            }
            if type_name == Token::Struct || type_name == Token::Const || type_name == Token::Comma {
                type_name = self.next_token(false)?;
                if type_name == Token::Struct || type_name == Token::Const {
                    type_name = self.next_token(false)?;
                }
            }
            let param_type = match type_name {
                Token::Type(t) => t,
                Token::Symbol(s) => DataType::Custom(s),
                tok => return Err(ParseError::ErrToken(tok))
            };
            let full_type = self.fill_data_type(param_type);
            
            let param_name = self.next_token(false)?.cast_sym()?;
           
            params.push(FuncParam {
                name:param_name,
                typ:full_type
            });
            if self.next_token(false)? == Token::RightParent {
                break;
            }
        }
        Ok(params)
    }

    fn parse_func_define(&mut self,mut tok_typ:Token) -> Result<FunctionDefine,ParseError> {
        self.skip_white();
        if tok_typ == Token::Struct || tok_typ == Token::Const {
            tok_typ = self.next_token(false)?;
            if tok_typ == Token::Struct || tok_typ == Token::Const {
                tok_typ = self.next_token(false)?; 
            };
        };

        let ret_type = match tok_typ {
            Token::Type(typ) => { typ },
            Token::Symbol(sym) => { DataType::Custom(sym) },
            tok => {return Err(ParseError::ErrToken(tok)); }
        };
        let full_type = self.fill_data_type(ret_type);
        //println!("ret:{:?}",ret_type);
        self.skip_white();
        let fn_name = self.next_token(true)?.cast_sym()?;
        self.lex_string.next();
        let params:Vec<FuncParam> = self.parse_params()?;
        
        Ok( FunctionDefine {
            ret_type:full_type,
            name:fn_name,
            params
        })
    }


    fn skip_white(&mut self) {
        self.skip_char_white();
        self.skip_comment();
        self.skip_char_white();
    }

    fn skip_char_white(&mut self) {
        while let Some(chr) = self.lex_string.lookahead(1) {
            if chr.is_whitespace() || chr == ';' {
                self.lex_string.next();
            } else {
                break;
            }
        }
    }

    

    fn skip_comment(&mut self) {
        let fst = self.lex_string.lookahead(1);
        let snd = self.lex_string.lookahead(2);
        if fst != Some('/') { return; }
        if snd == Some('*') {
            //println!("{:?}==",&snd);
            self.lex_string.next();
            self.lex_string.next();
            while let Some(chr) = self.lex_string.lookahead(1)  {
                if chr == '*' && self.lex_string.lookahead(2) == Some('/') {
                    self.lex_string.next();
                    self.lex_string.next();
                   
                    break;
                } else {
                    self.lex_string.next();
                }
            }
        } else if snd == Some('/') {
            self.lex_string.next();
            self.lex_string.next();
            
            self.lex_string.skip_white(|chr| !chr.is_whitespace());
        }
    }


    fn next_token(&mut self,only_sym:bool) -> Result<Token,ParseError> {
        if let Some(cache_tok) = self.cache_tokens.pop_front() {
            return Ok(cache_tok);
        }
        self.skip_white();
        
        if let Some(chr) = self.lex_string.lookahead(1) {
           let ret = match chr {
                '{' =>  Some(Token::LeftBrace),
                '}' => Some(Token::RightBrace),
                ',' => Some(Token::Comma),
                '=' => Some(Token::Eq),
                '(' => Some(Token::LeftParent),
                ')' =>Some(Token::RightParent),
                 _ => { None }
            };
            if let Some(tok) = ret {
                self.lex_string.next();
                return Ok(tok);
            }
        }
        let key_string = self.lex_string.take_while(|chr| 
            chr.is_whitespace() || chr == '(' || chr == ',' || chr == '(' || chr == ')' || chr == ';');
        
        if let Some(keyword) = key_string {
           if only_sym {
              return Ok(Token::Symbol(keyword.to_string()));
           }
           let tok = match keyword {
              "typedef" => Token::Typedef,
              "struct" => Token::Struct,
              "const" => Token::Const,
              "uint32_t" => Token::Type(DataType::U32),
              "uint64_t" => Token::Type(DataType::U64),
              "uint8_t" => Token::Type(DataType::U8),
              "float" => Token::Type(DataType::Float),
              "bool" => Token::Type(DataType::Bool),
              "String" => Token::Type(DataType::String),
              "void" => Token::Type(DataType::Void),
              "enum" => Token::Enum,
              
              sym_str => Token::Symbol(sym_str.to_string()),
           };
           return Ok(tok);
        }
        Err(ParseError::EOF)
    }

   


}

#[test]
fn test_parse() {
    let code_string = r#"typedef struct App App;
    typedef struct WindowConfig {
        float width;
        float height;
        WindowMode mode;
        bool vsync;
        struct String title;
    } WindowConfig;
   
    enum WindowMode {
        Windowed,
        BorderlessFullscreen,
        Fullscreen,
    };

    void core_add_module(uint8_t *app_ptr,String app_string);

    void app_set_fps(struct App *app_ptr, uint32_t fps);
    "#;
    let mut parser = FFIFileParser::new(code_string,"test".into());
    let stmt = parser.parse();
    dbg!(&stmt);
   
}