use sdl2::rect::Point;

use crate::engine::types::{MseEntityId, RenderState};

pub trait Renderable {
    fn render_offset<'texture, E>(&self, render_state: RenderState<'texture>, pos: Point, entity: hecs::Entity) -> ();
}
