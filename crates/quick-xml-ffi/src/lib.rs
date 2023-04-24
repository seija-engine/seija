use std::ffi::CString;

use quick_xml::events::attributes::Attribute;



#[test]
fn test() {
    use quick_xml::*;
    use quick_xml::events::*;
    let xml_str = r#"
        <!--123-->
        <CheckBox width = "10">
            <Template>
              <Em e="123" e2="true" />
            </Template>
        </CheckBox>
    "#;
    let mut reader = Reader::from_str(xml_str);
    reader.trim_text(true);
   
    loop {
        match reader.read_event() {
            Err(err) => panic!("err:{:?}",err),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
               
            },
            Ok(Event::Empty(e)) => {
                println!("empty ={:?}",e.name());
                for attrs in e.attributes() {
                    match attrs {
                        Ok(attr) => {
                            println!("attr ={:?}",attr);
                        },
                        Err(err) => panic!("err:{:?}",err),
                    }
                }
            },
            Ok(Event::End(e)) => {
                println!("end ={:?}",e.name());
            },
            Ok(Event::Text(text)) => {
                println!("text ={:?}",text);
            },
            Ok(Event::Comment(comment)) => {
                println!("comment ={:?}",comment);
            },
            _ => {}  
        }
    }
}