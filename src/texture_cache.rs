extern crate sdl2;

use sdl2::render::TextureCreator;
use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window, WindowContext};
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;

use lru::LruCache;
use std::num::{NonZero, NonZeroUsize};

use crate::constants::TILE_SIZE;

// mod critter;

pub struct EntityTextureManager<'texture, E>
where
{
    texture_creator: &'texture  TextureCreator<WindowContext>,
    texture_cache: LruCache<E, Rc<Texture<'texture>>>,
}

impl <'texture,E> EntityTextureManager<'texture, E>
where
    E: Hash + Eq + RendersToCanvas + Copy
{
    pub fn new<'a>(tc: &'a TextureCreator<WindowContext>, max_tiles: usize) -> Self
    where 'a : 'texture
    {
        EntityTextureManager{texture_creator : tc, texture_cache: LruCache::new(NonZero::new(max_tiles).unwrap())}
    }

    fn new_texture<'a>(&'a self) -> Texture<'texture> {
        self.texture_creator.create_texture_target(None, TILE_SIZE as _, TILE_SIZE as _).unwrap()
    }

    pub fn get_texture<'a,'b>(&'a mut self, canvas: &'b mut Canvas<Window>, tile: &E) -> Rc<Texture<'texture>> {
        let r = LruCache::get(&mut self.texture_cache, tile);
        if r.is_some() {
            return r.unwrap().clone();
        }

        let mut new_texture = self.new_texture();

        canvas.with_texture_canvas(&mut new_texture, |c| {tile.render_to_canvas(c);});
        let t_rc = Rc::new(new_texture);
        self.texture_cache.put(*tile, t_rc.clone());
        return t_rc;
    }
}

pub trait RendersToCanvas {
    fn render_to_canvas<'a>(&self, canvas: &'a mut Canvas<Window>);
}

trait MakeTileTexture<'texture> {
    fn new_texture<'a>(&'a self) -> Texture<'texture>;
}

impl <'texture,E> MakeTileTexture<'texture> for EntityTextureManager<'texture,E>
    where
    E: Hash + Eq + RendersToCanvas + Copy {
    fn new_texture<'a>(&'a self) -> Texture<'texture> {
        EntityTextureManager::new_texture(&self)
    }
}
