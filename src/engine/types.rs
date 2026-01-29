use hecs;
use sdl2::rect::Point;

use crate::constants::TILE_SIZE;

pub struct MseEntityId(u64);

pub struct RenderState {
    cam_pos: Point,
}

struct EngineState<'a> {
    render_state: RenderState,
    entities: &'a mut hecs::World,
}

pub struct TextureRef (u64);
