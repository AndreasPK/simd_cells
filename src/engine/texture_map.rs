extern crate sdl2;

use sdl2::render::TextureCreator;
use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window, WindowContext};
use sdl2::{pixels::PixelFormatEnum, surface::Surface};
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::constants::TILE_SIZE;

// mod critter;

pub struct TextureMap<'texture, E> {
    texture_creator: &'texture TextureCreator<WindowContext>,
    texture_cache: HashMap<E, Rc<Texture<'texture>>>,
}

impl<'texture, E> TextureMap<'texture, E>
where
    E: Hash + Eq + Copy,
{
    pub fn new<'a>(tc: &'a TextureCreator<WindowContext>) -> Self
    where
        'a: 'texture,
    {
        TextureMap {
            texture_creator: tc,
            texture_cache: HashMap::new(),
        }
    }

    // fn new_texture<'a>(&'a self) -> Texture<'texture> {
    //     self.texture_creator
    //         .create_texture_target(None, TILE_SIZE as _, TILE_SIZE as _)
    //         .unwrap()
    // }

    pub fn get_texture<'a, 'b>(
        &'a mut self,
        canvas: &'b mut Canvas<Window>,
        tile: &E,
    ) -> Rc<Texture<'texture>> {
        self.texture_cache.get(tile).unwrap().clone()

    }

    pub fn load_bmp<P: AsRef<std::path::Path>>(&mut self, path: P, key: E) -> () {
        let mut surface = Surface::load_bmp(path).unwrap();
        if surface.pixel_format_enum() != PixelFormatEnum::RGB24 {
            surface = surface.convert_format(PixelFormatEnum::RGB24).unwrap();
        }

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        self.texture_cache.insert(key, Rc::new(texture));
    }
}

