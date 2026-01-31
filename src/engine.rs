use sdl2::rect::Point;

use crate::engine::types::{EngineState, TextureRef};

mod traits;
pub mod types;
mod entities;
mod loader;
pub mod texture_cache;
pub mod texture_map;

impl <'texture>EngineState<'texture, TextureRef> {
    pub fn init() -> Self{
        let render_state = todo!();
        let entities = todo!();
        EngineState{
            render_state:render_state, entities:entities
        }
    }

}
// fn foo () {
//     traits::traits::
// }
