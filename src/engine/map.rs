#![allow(unused)]
#![allow(unused_imports)]

use std::vec;

use crate::constants::TILE_SIZE;
use crate::engine::texture_cache::{EntityTextureManager, RendersToCanvas};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
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
    /// 0..width
    width: usize,
    ///0..height (0 = top most)
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

    /// Get width of current grid
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Get height of current grid
    pub fn get_height(&self) -> usize {
        if self.width == 0 {
            0
        } else {
            self.tiles.len() / self.width
        }
    }

    /// Get tile at specific position
    pub fn get_tile_at(&self, x: usize, y: usize) -> Option<&Tile> {
        let height = self.get_height();
        if x >= self.width || y >= height {
            return None;
        }
        let idx = self.to_index(x, y);
        self.tiles.get(idx)
    }

    pub fn get_random_pos(&self) -> Point {
        debug_assert!(self.width > 0);
        debug_assert!(self.tiles.len() % self.width == 0);

        let height = self.tiles.len() / self.width;
        let x = fastrand::usize(..self.width);
        let y = fastrand::usize(..height);
        Point::new(x as i32, y as i32)
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
