extern crate sdl2;

use sdl2::render::TextureCreator;
use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window, WindowContext};
use sdl2::image::LoadSurface;
use sdl2::{pixels::PixelFormatEnum, surface::Surface};
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;

use crate::constants::TILE_SIZE;
use crate::engine::types::TextureRef;

// mod critter;

pub struct TextureMap<'texture, E>
{
    texture_creator: &'texture TextureCreator<WindowContext>,
    texture_map: HashMap<E, Rc<Texture<'texture>>>,
    file_cache: HashMap<PathBuf, E>,
    next: TextureRef,
}

impl<'texture, E> TextureMap<'texture, E>
where
    E: Hash + Eq,
{
    pub fn new<'a>(tc: &'a TextureCreator<WindowContext>) -> Self
    where
        'a: 'texture,
    {
        TextureMap {
            texture_creator: tc,
            texture_map: HashMap::new(),
            file_cache: HashMap::new(),
            next: TextureRef::new(),
        }
    }

    pub fn get_texture<'a, 'b>(
        &'a mut self,
        canvas: &'b mut Canvas<Window>,
        tile: &E,
    ) -> Rc<Texture<'texture>> {
        self.texture_map.get(tile).unwrap().clone()

    }

    pub fn load_bmp<P: AsRef<std::path::Path>>(&mut self, path: P, key: E) -> () {
        self.load_texture(path, key);
    }

    pub fn load_texture<P: AsRef<std::path::Path>>(&mut self, path: P, key: E) -> () {
        let mut surface = Surface::from_file(path).unwrap();
        if surface.pixel_format_enum() != PixelFormatEnum::RGB24 {
            surface = surface.convert_format(PixelFormatEnum::RGB24).unwrap();
        }

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        self.texture_map.insert(key, Rc::new(texture));
    }

}

impl<'texture> TextureMap<'texture, TextureRef> {
    pub fn load_texture_cached<P: AsRef<std::path::Path>>(&mut self, path: P) -> TextureRef {
        let path_buf = path.as_ref().to_path_buf();
        if let Some(existing) = self.file_cache.get(&path_buf) {
            return existing.clone();
        }

        let key = self.next.clone();
        self.next = key.clone().next();
        self.load_texture(&path_buf, key.clone());
        self.file_cache.insert(path_buf, key.clone());
        key
    }
}

