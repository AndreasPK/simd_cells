use std::path::Path;

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
    pub fn init<>(creator: &'texture sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self{
        let texture_map: Box<TextureMap<'texture, TextureRef>> = Box::new(TextureMap::new(creator));
        RenderState { cam_pos : Point::new(0,0), entity_texture_manager : texture_map }
    }

    pub fn texture_map_mut(&mut self) -> &mut TextureMap<'texture, TextureRef> {
        &mut self.entity_texture_manager
    }
}

pub struct EngineState<'texture> {
    pub render_state: RenderState<'texture>,
    pub entities: Box<hecs::World>,
}

#[derive(Hash,Eq,Debug,PartialEq,Clone)]
pub struct TextureRef(u32);

impl TextureRef{
    pub fn new() -> Self {
        TextureRef(0)
    }

    pub fn next(self) -> Self {
        TextureRef(self.0 + 1)
    }
}
