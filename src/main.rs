extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;

mod game;
mod texture_templates;
mod textures;

use crate::game::{Game, GameObject, CANVAS_HEIGHT, CANVAS_WIDTH, PIXEL_SIZE};
use crate::textures::textures;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Space Invaders: Rust", CANVAS_WIDTH, CANVAS_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let game_object = GameObject::new(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2);
    let mut game = Game::new(vec![game_object]);

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let (square_texture1, square_texture2) = textures(&mut canvas, &texture_creator)?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for object in &mut game.game_objects {
            canvas.copy(
                &square_texture1,
                None,
                Rect::new(object.x as i32, object.y as i32, PIXEL_SIZE, PIXEL_SIZE),
            )?;
        }

        canvas.present();

        game.update();
    }

    Ok(())
}
