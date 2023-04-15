use seija_render::RenderConfig;

use crate::create_pbr_plugin;

#[no_mangle]
pub unsafe fn render_config_add_pbr_plugin(config_ptr:&mut RenderConfig) {
    config_ptr.plugins.push(create_pbr_plugin()); 
}