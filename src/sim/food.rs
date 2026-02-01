use std::ops::{Index, IndexMut};

use hecs::Entity;

use crate::engine::entities::{RenderTileC, WorldPositionC};
use crate::engine::traits;
use crate::engine::types::EngineState;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum FoodType {
    Food0 = 0,
    Food1 = 1,
    Food2 = 2,
}

impl FoodType {
    pub fn texture_path(&self) -> &'static str {
        match self {
            FoodType::Food1 => "data/textures/Food1.bmp",
            FoodType::Food0 | FoodType::Food2 => "data/textures/Placeholder.bmp",
        }
    }
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

#[derive(hecs::Bundle, Copy, Clone, Debug)]
pub struct FoodC {
    pub kind: FoodType,
    pub amount: u8,
}

impl FoodC {
    pub fn new(kind: FoodType, amount: u8) -> Self {
        FoodC { kind, amount }
    }
}

#[derive(hecs::Bundle)]
pub struct FoodE(Entity);

impl FoodE {
    fn spawn(
        engine_state: &mut EngineState,
        position: WorldPositionC,
        renderable: RenderTileC,
        food: FoodC,
    ) -> Self {
        FoodE(engine_state.spawn_entity((position, renderable, food)))
    }

    pub fn spawn_at(sim: &mut EngineState<'_>, x: i32, y: i32, kind: FoodType, amount: u8) -> FoodE {
        let render_tile = RenderTileC::from_path(sim, kind.texture_path());
        let food = FoodC::new(kind, amount);
        FoodE::spawn(sim, WorldPositionC::new(x, y), render_tile, food)
    }

    pub fn components(
        loader: &mut impl traits::TextureLoader,
        x: i32,
        y: i32,
        kind: FoodType,
        amount: u8,
    ) -> (WorldPositionC, RenderTileC, FoodC) {
        let render_tile = RenderTileC::from_path(loader, kind.texture_path());
        let position = WorldPositionC::new(x, y);
        let food = FoodC::new(kind, amount);
        (position, render_tile, food)
    }
}
