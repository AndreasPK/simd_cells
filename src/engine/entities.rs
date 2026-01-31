////////////////////////////////////////////////////////////////////////////////
/// Critters <3
////////////////////////////////////////////////////////////////////////////////
use std::{rc::Rc, sync::Arc};

use hecs;
use sdl2::{rect::Point, render::Texture};

use crate::constants::TILE_SIZE;
use crate::engine::types;

#[derive(hecs::Bundle)]

struct WorldPositionC(Point);
#[derive(hecs::Bundle)]
struct SizeC(Point);

#[derive(hecs::Bundle)]
pub struct RenderTileC {
    texture: Arc<types::TextureRef>,
}

#[derive(hecs::Bundle)]
pub struct MovingC {
    speed: Point,
    //x+y
    direction: Point,
}

impl MovingC {
    fn new(speed: Point, direction: Point) -> Self {
        MovingC { speed, direction }
    }
}

pub mod food {
    use hecs::Entity;

    use crate::engine::entities::WorldPositionC;

    #[derive(hecs::Bundle)]

    struct FoodE(Entity);

    impl FoodE {

        fn spawn(
                self,
                world: &mut hecs::World,
                position: WorldPositionC,
                renderable: super::RenderTileC,
            ) -> Self {
                FoodE(world.spawn((position,renderable)))
        }
    }

}

pub mod critter {
    use sdl2::rect::Point;

    use crate::{constants::TILE_SIZE, engine::entities::WorldPositionC};

    #[derive(hecs::Bundle)]
    struct CritterStatsC {
        energy: u32,
        /// in seconds
        lifetime: u32,
    }

    impl CritterStatsC {
        fn new(energy: u32, lifetime: u32) -> Self {
            CritterStatsC { energy, lifetime }
        }
        fn new_new(energy: u32) -> Self {
            CritterStatsC {
                energy,
                lifetime: 0,
            }
        }
    }

    struct CritterE(hecs::Entity);

    impl CritterE {
        fn spawn(
            self,
            world: &mut hecs::World,
            position: Point,
            direction: Point,
            energy: u32,
        ) -> Self {
            // The builder pattern:

            let components = (
                WorldPositionC(position),
                super::MovingC::new(Point::new(0, 0), direction),
                super::SizeC(Point::new(TILE_SIZE as _, TILE_SIZE as _)),
                CritterStatsC::new_new(energy),
            );

            let e: hecs::Entity = world.spawn(components);

            CritterE(e)

            // let size = SizeC()
        }
    }
}
