use std::path::Path;

use hecs::World;
use sdl2::{rect::Point, render::TextureCreator, video::WindowContext};

use crate::engine::types::{EngineState, RenderState, TextureRef};

mod traits;
pub mod types;
mod entities;
mod loader;
pub mod texture_cache;
pub mod texture_map;

impl <'texture>EngineState<'texture> {
    pub fn init(texture_creator: &'texture TextureCreator<WindowContext>) -> Self{
        let render_state: RenderState<'texture> = RenderState::init(texture_creator);
        let entities: World = World::new();
        EngineState{
            render_state:render_state, entities:Box::new(entities)
        }
    }

    pub fn load_textures_from_folder(&mut self, path: &Path) -> Option<()> {
        if !path.exists() {
            eprintln!("Texture folder does not exist: {:?}", path);
            return None;
        }

        let entries = std::fs::read_dir(path).ok()?;
        for entry in entries {
            let entry = entry.ok()?;
            let file_path = entry.path();
            if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("bmp") || ext.eq_ignore_ascii_case("png") {
                    self.render_state
                        .texture_map_mut()
                        .load_texture_cached(&file_path);
                }
            }
        }
        Some(())
    }

}
// fn foo () {
//     traits::traits::
// }
