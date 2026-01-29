#![allow(unused)]
#![allow(unused_imports)]

use std::{
    num::Wrapping, ops::{Add, Index, IndexMut}, vec
};

use crate::constants::TILE_SIZE;
use crate::texture_cache::{EntityTextureManager, RendersToCanvas};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator, WindowCanvas},
    video::{Window, WindowContext},
};

#[derive(Copy,Clone)]
pub enum FoodType {
    Food0 = 0,
    Food1 = 1,
    Food2 = 2,
}

impl<T> Index<FoodType> for [T] {
    type Output = T;
    fn index(&self, idx: FoodType) -> &Self::Output {
        match idx {
            FoodType::Food0 => &self[0],
            FoodType::Food1 => &self[1],
            FoodType::Food2 => &self[2],
        }
    }
}

impl<T> IndexMut<FoodType> for [T] {
    fn index_mut(&mut self, idx: FoodType) -> &mut Self::Output {
        match idx {
            FoodType::Food0 => &mut self[0],
            FoodType::Food1 => &mut self[1],
            FoodType::Food2 => &mut self[2],
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Tile {
    bitmap: u64,
    food: [u8; 4],
}

impl Tile {
    //Assumes 32x32
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(self.food[0], self.food[1], self.food[2]));
        canvas.set_draw_color(Color::RGB(self.food[0], self.food[1], self.food[2]));
        canvas.fill_rect(None);
    }
}

pub const EMPTY_TILE: Tile = Tile {
    bitmap: 0,
    food: [0, 0, 0, 0],
};

pub struct GridMap<'map> {
    width: usize,
    tiles: Vec<Tile>,
    entity_texture_manager: EntityTextureManager<'map, Tile>,
}

impl<'map> GridMap<'map> {
    #[allow(dead_code)]
    pub fn new(
        width: usize,
        height: usize,
        mgr: EntityTextureManager<'map, Tile>,
    ) -> GridMap<'map> {
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

    pub fn add_food(&mut self, x:usize , y:usize, ft : FoodType, amount: u8) {
        let idx = self.to_index(x, y);
        self.tiles[idx].food[ft] = self.tiles[idx].food[ft].wrapping_add(amount);
    }

    pub fn render_grid<'a,'b> (&'b mut self, canvas: &'a mut WindowCanvas)
        where
        {
        for (idx, tile) in self.tiles.iter().enumerate() {
            let x = idx % self.width;
            let y = idx / self.width;

            let t = self.entity_texture_manager.get_texture(canvas, tile);

            canvas.copy(t.as_ref(), None, Rect::new((x*TILE_SIZE) as _, (y*TILE_SIZE) as _, TILE_SIZE as _, TILE_SIZE as _)).unwrap();

        }
    }
}

impl RendersToCanvas for Tile {
    fn render_to_canvas(&self, canvas: &mut Canvas<Window>) {
        self.render(canvas);
    }
}
