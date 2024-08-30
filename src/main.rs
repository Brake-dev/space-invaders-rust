extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

mod game;
mod texture_templates;
mod textures;

use crate::game::{Game, Player, CANVAS_HEIGHT, CANVAS_WIDTH, PIXEL_SIZE};
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

    let mut game = Game::new();
    let mut player = Player::new();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let (textures, missing_texture) = textures(&mut canvas, &texture_creator)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut prev_keys = HashSet::new();

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

        let keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let new_keys = &keys - &prev_keys;
        let old_keys = &prev_keys - &keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            if new_keys.contains(&Keycode::Left) {
                player.set_moving_left(true);
            } else if old_keys.contains(&Keycode::Left) {
                player.set_moving_left(false);
            }

            if new_keys.contains(&Keycode::Right) {
                player.set_moving_right(true);
            } else if old_keys.contains(&Keycode::Right) {
                player.set_moving_right(false);
            }

            if new_keys.contains(&Keycode::Space) {
                player.shoot();
            }
        }

        prev_keys = keys;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        for row in game.get_all_textures() {
            for object in row {
                canvas.copy(
                    match textures.get(&object.texture_name) {
                        Some(tex) => tex,
                        None => &missing_texture,
                    },
                    None,
                    Rect::new(
                        object.x as i32,
                        object.y as i32,
                        PIXEL_SIZE * object.width,
                        PIXEL_SIZE * object.height,
                    ),
                )?;
            }
        }

        canvas.copy(
            match textures.get(&player.game_object.texture_name) {
                Some(tex) => tex,
                None => &missing_texture,
            },
            None,
            Rect::new(
                player.game_object.x as i32,
                player.game_object.y as i32,
                PIXEL_SIZE * player.game_object.width,
                PIXEL_SIZE * player.game_object.height,
            ),
        )?;

        for bullet in &player.bullets {
            canvas.copy(
                match textures.get(&String::from("shot_texture")) {
                    Some(tex) => tex,
                    None => &missing_texture,
                },
                None,
                Rect::new(
                    bullet.x as i32,
                    bullet.y as i32,
                    PIXEL_SIZE * bullet.width,
                    PIXEL_SIZE * bullet.height,
                ),
            )?;
        }

        canvas.present();

        game.update();
        player.update();
    }

    Ok(())
}
