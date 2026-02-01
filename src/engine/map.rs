#![allow(unused)]
#![allow(unused_imports)]

use std::vec;

use crate::constants::TILE_SIZE;
use crate::engine::texture_cache::{EntityTextureManager, RendersToCanvas};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, WindowCanvas},
    video::Window,
};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Tile {
    bitmap: u64,
}

impl Tile {
    // Assumes 32x32
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let bytes = self.bitmap.to_le_bytes();
        canvas.set_draw_color(Color::RGB(bytes[0], bytes[1], bytes[2]));
        canvas.fill_rect(None);
    }
}

pub const EMPTY_TILE: Tile = Tile {
    bitmap: 0,
};

pub struct GridMap<'map> {
    width: usize,
    tiles: Vec<Tile>,
    entity_texture_manager: EntityTextureManager<'map, Tile>,
}

impl<'map> GridMap<'map> {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize, mgr: EntityTextureManager<'map, Tile>) -> GridMap<'map> {
        let size: usize = (width * height).try_into().unwrap();
        let tiles: Vec<Tile> = vec![EMPTY_TILE; size];
        GridMap {
            tiles,
            width: width,
            entity_texture_manager: mgr,
        }
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn render_grid<'a, 'b>(&'b mut self, canvas: &'a mut WindowCanvas)
    where
    {
        for (idx, tile) in self.tiles.iter().enumerate() {
            let x = idx % self.width;
            let y = idx / self.width;

            let t = self.entity_texture_manager.get_texture(canvas, tile);

            canvas
                .copy(
                    t.as_ref(),
                    None,
                    Rect::new(
                        (x * TILE_SIZE) as _,
                        (y * TILE_SIZE) as _,
                        TILE_SIZE as _,
                        TILE_SIZE as _,
                    ),
                )
                .unwrap();
        }
    }
}

impl RendersToCanvas for Tile {
    fn render_to_canvas(&self, canvas: &mut Canvas<Window>) {
        self.render(canvas);
    }
}
