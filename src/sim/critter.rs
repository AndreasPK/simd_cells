use std::path::Path;

use sdl2::image;
use sdl2::rect::Point;

use crate::constants::TILE_SIZE;
use crate::engine::entities::{MovingC, RenderTileC, SizeC, WorldPositionC};
use crate::engine::traits;

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

    fn default() -> Self {
        CritterStatsC { energy: 0, lifetime: 0 }
    }
}

struct CritterE(hecs::Entity);

type CritterComponents = (WorldPositionC, MovingC, RenderTileC, CritterStatsC);

impl CritterE {
    fn spawn(
        loader: &mut impl traits::TextureLoader,
        world: &mut hecs::World,
        p: &impl AsRef<Path>,
        position: Point,
        direction: Point,
        energy: u32,
    ) -> Self {
        let tile = RenderTileC::from_path(loader, p);
        let components: CritterComponents = (
            WorldPositionC(position),
            MovingC::new(Point::new(0, 0), direction),
            tile,
            CritterStatsC::new_new(energy),
        );

        let e: hecs::Entity = world.spawn(components);

        CritterE(e)
    }

    pub fn new_with_texture_path(
        loader: &mut impl traits::TextureLoader,
        x: i32,
        y: i32,
        p: &impl AsRef<Path>
    ) -> CritterComponents {
        let render_tile = RenderTileC::from_path(loader, p);
        let position = WorldPositionC::new(x, y);
        let move_c = MovingC::inert();
        let critter_stats = CritterStatsC::new(100, 0);
        (position,move_c,render_tile,critter_stats)
    }
}
