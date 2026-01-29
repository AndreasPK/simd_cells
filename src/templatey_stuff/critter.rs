extern crate sdl2;

use sdl2::sys::{SDL_RenderDrawPoint, SDL_TextureAccess};
use sdl2::video::WindowContext;
use sdl2::{pixels::Color, video::Window};
use sdl2::rect::{self, Point};


use sdl2::render::{self, Canvas, Texture, TextureCreator, WindowCanvas, RenderTarget};

// pub struct Critter<'r>{
//     pub id: u64,
//     pub texture: Texture<'r>
// }


// pub fn mk_critter<'r>(canvas:  &mut Canvas<Window>, creator : &'r TextureCreator<WindowContext>) -> Critter<'r> {
//     // let tex_creater: render::TextureCreator<sdl2::video::WindowContext> = canvas.texture_creator();
//     let tex = creator.create_texture(None, render::TextureAccess::Target, 16, 16);
//     let mut tex2: Texture<'_> = tex.unwrap();

//     let _ = canvas.with_texture_canvas(&mut tex2, | canv: &mut Canvas<Window> | {
//             canv.set_draw_color(sdl2::pixels::Color::BLUE);
//             canv.draw_point(sdl2::rect::Point::new(1, 1)).unwrap();
//             canv.fill_rect(sdl2::rect::Rect::new(2, 2,14,14)).unwrap();

//             return;
//         });

//     // SDL_RenderDrawPoint(renderer, x, y)
//     return Critter{ id: 1, texture: tex2};
// }

