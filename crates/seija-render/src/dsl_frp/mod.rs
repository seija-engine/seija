mod frp_comp;
mod system;
mod fns;
mod builder;
mod elems;
mod plugin;
mod errors;
mod win_event;
mod ubo_array_collect;
pub mod render_path;
mod post_stack;
use seija_app::App;
pub use elems::ibl_node::IBLEnv;
pub use system::FRPDSLSystem;
pub use plugin::RenderScriptPlugin;
pub use elems::IUpdateNode;
pub use ubo_array_collect::UBOArrayCollect;
pub use lite_clojure_frp::FRPSystem;
pub use post_stack::PostEffectStack;

//use self::post_stack::post_material_to_global;


pub fn init_dsl_frp(_app:&mut App) {
    //app.add_system(CoreStage::PostUpdate, post_material_to_global);
}