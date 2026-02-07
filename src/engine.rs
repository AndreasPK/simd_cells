use std::path::Path;
use std::time::Instant;

use hecs::{DynamicBundle, Entity, World};
use sdl2::render::{Canvas, TextureCreator, WindowCanvas};
use sdl2::video::{Window, WindowContext};

use crate::engine::{
    entities::{RenderTileC, WorldPositionC},
    map::GridMap,
    texture_cache::EntityTextureManager,
    traits::TextureLoader,
    types::{EngineState, RenderState, TextureRef},
};

pub mod entities;
mod loader;
pub mod map;
pub mod texture_cache;
pub mod texture_map;
pub mod traits;
pub mod types;

impl<'texture> EngineState<'texture> {
    pub fn init(
        texture_creator: &'texture TextureCreator<WindowContext>,
        width: usize,
        height: usize,
    ) -> Self {
        let render_state: RenderState<'texture> = RenderState::init(texture_creator);
        let entities: World = World::new();
        let max_tiles: usize = width * height;
        let tile_texture_mgr = EntityTextureManager::new(texture_creator, max_tiles);
        let grid: GridMap<'texture> = GridMap::new(width, height, tile_texture_mgr);
        EngineState {
            render_state: render_state,
            grid: grid,
            entities: Box::new(entities),
            tick: 0,
            start_time: Instant::now(),
        }
    }

    pub fn get_map(&self) -> &GridMap<'texture> {
        &self.grid
    }

    pub fn preload_textures_from_folder<P>(&mut self, p: &P) -> Option<()>
    where
        P: AsRef<Path>,
    {
        let path: &Path = p.as_ref();
        if !path.exists() {
            eprintln!("Texture folder does not exist: {:?}", path);
            return None;
        }

        let entries = std::fs::read_dir(path).ok()?;
        for entry in entries {
            let entry = entry.ok()?;
            let file_path = entry.path();
            if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("bmp") || ext.eq_ignore_ascii_case("png") {
                    self.render_state
                        .texture_map_mut()
                        .load_texture_cached(&file_path);
                }
            }
        }
        Some(())
    }

    pub fn spawn_entity(&mut self, components: impl DynamicBundle) -> Entity {
        self.entities.spawn(components)
    }

    pub fn load_texture_cached<P: AsRef<Path>>(&mut self, path: P) -> TextureRef {
        self.render_state
            .texture_map_mut()
            .load_texture_cached(path)
    }

    pub fn render_entities(&mut self, canvas: &mut Canvas<Window>) {
        let entities: &mut Box<World> = &mut self.entities;
        for (render_tile, pos) in entities.query::<(&RenderTileC, &WorldPositionC)>().iter() {
            self.render_state
                .render_tile(canvas, pos.clone(), *render_tile);
        }
    }

    pub fn render_map(&mut self, canvas: &mut WindowCanvas) {
        self.grid.render_grid(canvas);
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        self.render_map(canvas);
        self.render_entities(canvas);
    }
}

impl<'texture> TextureLoader for EngineState<'texture> {
    fn load_texture_cached<P>(&mut self, p: P) -> TextureRef
    where
        P: AsRef<Path>,
    {
        self.load_texture_cached(p)
    }
}
