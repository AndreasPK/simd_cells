use sdl2::rect::Point;

use crate::engine::types::{MseEntityId, RenderState};

pub trait Renderable {
    fn render_offset(&self, render_state: RenderState, pos: Point, entity: hecs::Entity) -> ();
}