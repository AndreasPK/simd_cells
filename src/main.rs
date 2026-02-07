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

use crate::engine::types::EngineState;
mod constants;
mod engine;
mod sim;

use engine::map;
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

    let mut window = video_subsystem
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

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut engine_state: EngineState = EngineState::init(&texture_creator, width, height);

    engine_state.preload_textures_from_folder(&"data/textures");
    sim::init_sim(&mut engine_state);




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


        // let x = fastrand::i32(0..width as i32);
        // let y = fastrand::i32(0..height as i32);
        // sim::food::FoodE::spawn_at(&mut engine_state, x, y, sim::food::FoodType::Food1, 1);
        // engine_state.render_map(&mut canvas);

        engine_state.render(&mut canvas);

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

            let target = Rect::new(50, 50, width/2, height/2);
            canvas.copy(&texture, None, target).unwrap();
        }

        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
