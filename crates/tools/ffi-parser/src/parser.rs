use crate::{lex_string::LexString, ffi_file::FFIFile};
use crate::ffi_file::{Stmt,DataType,StructType, StructItem, FunctionDefine};
#[derive(Debug)]
pub enum ParseError {
    EOF,
    ErrType(Token),
    ErrToken(Token),
    ErrFnName
}

#[derive(Debug,PartialEq, Eq)]
enum Token {
    Type(DataType),
    Symbol(String),
    Keyword(String),
    Typedef,
    Struct,
    Enum,
    LeftBrace,
    RightBrace
}




pub struct FFIFileParser<'a> {
    cache_tokens:Vec<Token>,
    lex_string:LexString<'a>
}

impl<'a> FFIFileParser<'a> {
    pub fn new(file_source:&'a str) -> Self {
        let lexer = LexString::new(file_source, 5);
        FFIFileParser { cache_tokens:vec![],lex_string:lexer }
    }

    pub fn parse(&mut self) -> Result<FFIFile,ParseError> {
        let mut stmt_list:Vec<Stmt> = vec![];
        while let Ok(v) = self.parse_item() {
           
            stmt_list.push(v);
        }

        Ok(FFIFile { stmts:stmt_list} )
    }

    fn parse_item(&mut self) -> Result<Stmt,ParseError> {
       let tok = self.next_token(false)?;
       match tok {
        Token::Typedef => {
            let next = self.next_token(false)?;
            if next == Token::Struct {
                let new_name = self.next_keyword()?;
                let next_tok = self.next_keyword()?;
                if next_tok == "{" {
                    let struct_type = self.parse_struct()?;
                    return Ok(Stmt::TypedefStruct(new_name,struct_type));
                } else {
                   return Ok(Stmt::TypedefStructName(new_name,next_tok));
                }
            } else if let Token::Type(typ) = next {
                let new_name = self.next_keyword()?;
                return Ok(Stmt::Typedef(typ,new_name))
            } else {
                return Err(ParseError::ErrToken(next));
            }
        },
        Token::Enum => {
            self.parse_enum();
            todo!()
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
            let item_mame = self.next_keyword()?;
            let item = StructItem { typ:real_type,name:item_mame };
            items.push(item);
        }
        if let Some(_) = self.lex_string.take_while(|chr| chr== ';') {
            self.lex_string.next();
        }
        let struct_type = StructType { items };
        Ok(struct_type)
    }

    fn parse_enum(&mut self) {

    }

    fn parse_func_define(&mut self,mut tok_typ:Token) -> Result<FunctionDefine,ParseError> {
        if tok_typ == Token::Struct {tok_typ = self.next_token(false)?; };
        let ret_type = match tok_typ {
            Token::Type(typ) => { typ },
            Token::Symbol(sym) => { DataType::Custom(sym) },
            tok => {return Err(ParseError::ErrToken(tok)); }
        };
        self.skip_white();
        let fn_name = self.lex_string.take_while(|chr| chr == '(')
                               .ok_or(ParseError::ErrFnName)?.to_string();
        self.lex_string.next();

        loop {
            let mut type_tok = self.next_token(false)?;
            if type_tok == Token::Struct {
                type_tok = self.next_token(false)?;
            }
        }
        println!("{:?} {}",ret_type,fn_name);
        todo!()
    }
    
    fn next_keyword(&mut self) -> Result<String,ParseError> {
        self.skip_white();
        self.lex_string.take_while(|chr| chr.is_whitespace() || chr == ';')
                       .map(|v|v.to_string()).ok_or(ParseError::EOF)
    }

    fn skip_white(&mut self) {
        while let Some(chr) = self.lex_string.lookahead(1) {
            if chr.is_whitespace() || chr == ';' {
                self.lex_string.next();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self,only_sym:bool) -> Result<Token,ParseError> {
        self.skip_white();
        let key_string = self.lex_string.take_while(|chr| chr.is_whitespace());
        if let Some(keyword) = key_string {
           if only_sym {
              return Ok(Token::Symbol(keyword.to_string()));
           }
           let tok = match keyword {
              "typedef" => Token::Typedef,
              "struct" => Token::Struct,
              "uint32_t" => Token::Type(DataType::U32),
              "uint8_t" => Token::Type(DataType::U8),
              "float" => Token::Type(DataType::Float),
              "bool" => Token::Type(DataType::Bool),
              "String" => Token::Type(DataType::String),
              "void" => Token::Type(DataType::Void),
              "{" => Token::LeftBrace,
              "}" => Token::RightBrace,
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
    void core_add_module(uint8_t *app_ptr);
    "#;
    let mut parser = FFIFileParser::new(code_string);
    let stmt = parser.parse();
    dbg!(&stmt);
   
}