////////////////////////////////////////////////////////////////////////////////
/// Critters <3
////////////////////////////////////////////////////////////////////////////////
use std::{path::Path, sync::Arc};

use hecs::{self, DynamicBundle};
use sdl2::rect::Point;

use crate::{
    constants::TILE_SIZE,
    engine::types::{self, TextureRef},
};

pub trait EntityComponents {
    type Components : DynamicBundle;
}

#[derive(hecs::Bundle,Clone)]

pub struct WorldPositionC(pub Point);
impl WorldPositionC {
    pub fn new(x: i32, y: i32) -> Self {
        WorldPositionC(Point::new(x, y))
    }
}
#[derive(hecs::Bundle)]
pub struct SizeC(pub Point);

impl SizeC {
    pub fn tile_sized() -> Self {
        SizeC(Point::new(TILE_SIZE as _, TILE_SIZE as _))
    }
}

#[derive(hecs::Bundle, Copy, Clone)]
pub struct RenderTileC {
    pub texture: types::TextureRef,
}

impl RenderTileC {
    pub fn new(t: TextureRef) -> Self {
        RenderTileC { texture: t }
    }
    pub fn from_path<P: AsRef<Path>>(engine: &mut impl super::traits::TextureLoader, p: P) -> Self {
        RenderTileC {
            texture: engine.load_texture_cached(p.as_ref()),
        }
    }
}

#[derive(hecs::Bundle)]
pub struct MovingC {
    speed: Point,
    //x+y
    orientation: Point,
}

impl MovingC {
    pub fn new(speed: Point, orientation: Point) -> Self {
        MovingC { speed, orientation }
    }

    pub fn inert() -> Self {
        MovingC { speed: Point::new(0,0), orientation: Point::new(0,0)}
    }
}
