use hecs;
use sdl2::rect::Point;

use crate::constants::TILE_SIZE;
use super::texture_map::TextureMap;

pub struct MseEntityId(u64);

pub struct RenderState<'texture> {
    cam_pos: Point,
    entity_texture_manager: Box<TextureMap<'texture, TextureRef>>,
}

impl <'texture>RenderState<'texture> {
    fn init<>(creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self{
        let texture_map: Box<TextureMap<'texture, _>> = Box::new(TextureMap::new(creator));
        RenderState { cam_pos = Point::new(0,0), entity_texture_manager : texture_map }
    }
}

pub struct EngineState<'texture, E> {
    pub render_state: RenderState<'texture, E>,
    pub entities: Box<hecs::World>,
}

#[derive(Hash,Eq,Debug,PartialEq)]
pub struct TextureRef (u64);
