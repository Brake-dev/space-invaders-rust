extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

mod game;
mod texture_templates;
mod textures;
mod util;

use crate::game::{Game, Player, CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::textures::textures;
use crate::util::{draw_texture, overlaps};

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

    let mut shot_timer = 0;

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

            if new_keys.contains(&Keycode::Space) && shot_timer == 0 {
                player.shoot();
                shot_timer = 20;
            }
        }

        prev_keys = keys;

        if shot_timer > 0 {
            shot_timer -= 1;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        for invader in game.get_all_invader_objects() {
            draw_texture(&mut canvas, &textures, &missing_texture, &invader)?;
        }

        for object in &game.barrier_row {
            draw_texture(&mut canvas, &textures, &missing_texture, &object)?;
        }

        draw_texture(
            &mut canvas,
            &textures,
            &missing_texture,
            &player.game_object,
        )?;

        for bullet in &player.bullets {
            draw_texture(&mut canvas, &textures, &missing_texture, &bullet)?;
        }

        canvas.present();

        let mut next_bullets = player.bullets.clone();
        let mut next_invaders = game.get_all_invader_objects();

        for bullet in &mut next_bullets {
            for invader in &mut next_invaders {
                if overlaps(&invader, &bullet) {
                    invader.is_destroyed = true;
                    bullet.is_destroyed = true;
                }
            }
        }

        player.bullets = next_bullets;
        game.set_all_invader_objects(next_invaders);

        game.update();
        player.update();
    }

    Ok(())
}
