use font_system::FontSystem;
mod font_system;
mod font;

const FILE_PATH:&'static str = "F:\\Project\\Rust\\seija\\seija-examples\\res\\ui\\WenQuanYiMicroHei.ttf";
#[test]
fn test() {
    let mut font_system = FontSystem::default();
    let mut db = fontdb::Database::new();

    db.load_system_fonts();
    for face in db.faces() {
        println!("face:{:?} ",face.source);
    }
}