#![allow(unused)]

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::sync::Arc;
// use sdl2::render::{Canvas, RenderTarget};
use fastrand;
use std::time::{Duration, Instant};

use crate::map::GridMap;
use crate::texture_cache::EntityTextureManager;
use crate::engine::texture_cache;
mod constants;
mod map;
mod engine;
// mod tmp;

pub fn main() {
    let width: usize = 20;
    let height: usize = 10;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf = sdl2::ttf::init().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(4, 5);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    /* create a new OpenGL context and make it current */
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    let gl_context = window.gl_create_context().unwrap();

    let glow_ctx = Arc::new(unsafe {
        glow::Context::from_loader_function(|name| {
            video_subsystem.gl_get_proc_address(name) as *const _
        })
    });

    let mut canvas = window.clone().into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let max_tiles: usize = width * height;

    let c: sdl2::render::TextureCreator<sdl2::video::WindowContext> = canvas.texture_creator();
    let tile_texture_mgr: EntityTextureManager<map::Tile> =
        texture_cache::EntityTextureManager::new(&c, max_tiles);

    let mut grid_map: GridMap<'_> = GridMap::new(width, height, tile_texture_mgr);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let font = ttf.load_font("cambria.ttf", 32).unwrap();

    // Convert surface → texture
    let texture_creator_font = canvas.texture_creator();

    let start = Instant::now();
    let mut iterations = 0;

    'running: loop {
        iterations += 1;
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::GRAY);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            };
        }


        grid_map.add_food(
            fastrand::usize(0..width),
            fastrand::usize(0..height),
            map::FoodType::Food0,
            1,
        );
        grid_map.render_grid(&mut canvas);

        {
            let fps = iterations / (Instant::now().duration_since(start).as_secs().max(1));
            let txt = format!("FPS:{fps}");
            // Render text to a surface
            let surface = font
                .render(txt.as_str())
                .blended(Color::RGB(255, 255, 255))
                .map_err(|e| e.to_string())
                .unwrap();

            let texture = texture_creator_font
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())
                .unwrap();
            let TextureQuery { width, height, .. } = texture.query();

            let target = Rect::new(50, 50, width, height);
            canvas.copy(&texture, None, target).unwrap();
        }

        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
