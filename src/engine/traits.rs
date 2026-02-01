use std::path::Path;

use sdl2::rect::Point;

use crate::engine::types::{MseEntityId, RenderState, TextureRef};

pub trait Renderable {
    fn render_offset<'texture, E>(&self, render_state: RenderState<'texture>, pos: Point, entity: hecs::Entity) -> ();
}

/// Expanding abstractions to cover the needs to the expanding abstractions.
pub(crate) trait TextureLoader {
    fn load_texture_cached<P:AsRef<Path>>(&mut self, p: P) -> TextureRef;
}