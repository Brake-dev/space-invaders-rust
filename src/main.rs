extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;

mod barrier;
mod game;
mod invader;
mod player;
mod texture_templates;
mod textures;
mod ufo;
mod ui;
mod util;

use crate::game::{Game, GameObject, State, CANVAS_HEIGHT, CANVAS_WIDTH, FPS, PIXEL_SIZE};
use crate::player::Player;
use crate::textures::textures;
use crate::ui::{create_ui, UI};
use crate::util::{draw_texture, draw_texture_nameless, overlaps};

pub struct RetryEvent;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Space Invaders: Rust",
            CANVAS_WIDTH as u32,
            CANVAS_HEIGHT as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(FPS)?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut game = Game::new();
    let mut player = Player::new();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let (textures, missing_texture) = textures(&mut canvas, &texture_creator)?;
    let (modal_hash, arrow_texture, ui_texture_hash, ui_targets_hash, default_target) =
        create_ui(&mut canvas, &texture_creator)?;

    let explosion_game_object = GameObject::new(
        player.game_object.rect.x,
        player.game_object.rect.y,
        12 * PIXEL_SIZE as u32,
        10 * PIXEL_SIZE as u32,
        String::from("explosion_texture"),
    );

    let mut ui = UI::new(
        match ui_targets_hash.get("retry") {
            Some(target) => *target,
            None => default_target,
        },
        match ui_targets_hash.get("quit") {
            Some(target) => *target,
            None => default_target,
        },
    );

    let event = sdl_context.event()?;
    let mut event_pump = sdl_context.event_pump()?;

    event.register_custom_event::<RetryEvent>()?;

    let mut player_explosion_timer = 0;
    let mut game_over_timer = 0;
    let mut ufo_timer = game.get_next_ufo_time();

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

            if event.is_user_event() {
                let retry = event.as_user_event_type::<RetryEvent>();
                match retry {
                    Some(_) => {
                        game = Game::new();
                        player = Player::new();

                        player_explosion_timer = 0;
                        game_over_timer = 0;
                        ufo_timer = game.get_next_ufo_time();
                    }
                    None => (),
                }
            }
        }

        let keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        fps_manager.delay();

        if game.state != State::Paused {
            for invader in &game.invaders {
                draw_texture(
                    &mut canvas,
                    &textures,
                    &missing_texture,
                    &invader.game_object,
                )?;
            }

            for shot in &game.invader_shots {
                draw_texture(&mut canvas, &textures, &missing_texture, &shot)?;
            }

            for explosion in &game.explosions {
                draw_texture(&mut canvas, &textures, &missing_texture, &explosion)?;
            }

            for barrier in &game.barrier_row {
                draw_texture(
                    &mut canvas,
                    &textures,
                    &missing_texture,
                    &barrier.game_object,
                )?;

                for collider in &barrier.colliders {
                    if collider.is_destroyed {
                        draw_texture_nameless(
                            &mut canvas,
                            match textures.get("barrier_mask_texture") {
                                Some(tex) => tex,
                                None => &missing_texture,
                            },
                            &Rect::new(
                                collider.rect.x,
                                collider.rect.y,
                                5 * PIXEL_SIZE as u32,
                                5 * PIXEL_SIZE as u32,
                            ),
                        )?;
                    }
                }
            }

            if !player.game_object.is_destroyed {
                draw_texture(
                    &mut canvas,
                    &textures,
                    &missing_texture,
                    &player.game_object,
                )?;
            } else {
                player_explosion_timer += 1;
                game_over_timer += 1;
            }

            if player_explosion_timer > 0 && player_explosion_timer < 10 {
                draw_texture(
                    &mut canvas,
                    &textures,
                    &missing_texture,
                    &explosion_game_object,
                )?;
            }

            if ufo_timer == 0 {
                game.toggle_spawn_ufo();
                ufo_timer = game.get_next_ufo_time();
            } else {
                ufo_timer -= 1;
            }

            if game.ufo_active {
                draw_texture(
                    &mut canvas,
                    &textures,
                    &missing_texture,
                    &game.ufo.game_object,
                )?;
            }

            if game_over_timer > 1 {
                game.toggle_state();
            }

            for bullet in &player.bullets {
                draw_texture(&mut canvas, &textures, &missing_texture, &bullet)?;
            }

            canvas.present();

            let mut next_bullets = player.bullets.clone();
            let mut next_invaders = game.get_all_invader_objects();
            let mut next_invader_shots = game.invader_shots.clone();
            let mut next_colliders = game.get_all_barrier_colliders();

            for bullet in &mut next_bullets {
                for invader in &mut next_invaders {
                    if overlaps(&invader.rect, &player.game_object.rect) {
                        player.game_object.is_destroyed = true;
                    }

                    if overlaps(&invader.rect, &bullet.rect) {
                        invader.is_destroyed = true;
                        bullet.is_destroyed = true;
                    }
                }

                for collider in &mut next_colliders {
                    if !collider.is_destroyed && overlaps(&collider.rect, &bullet.rect) {
                        collider.is_destroyed = true;
                        bullet.is_destroyed = true;
                    }
                }

                if overlaps(&game.ufo.game_object.rect, &bullet.rect) && game.ufo_active {
                    game.ufo.game_object.is_destroyed = true;
                    bullet.is_destroyed = true;
                }
            }

            for invader_shot in &mut next_invader_shots {
                if overlaps(&invader_shot.rect, &player.game_object.rect) {
                    player.game_object.is_destroyed = true;
                }

                for bullet in &mut next_bullets {
                    if overlaps(&invader_shot.rect, &bullet.rect) {
                        invader_shot.is_destroyed = true;
                        bullet.is_destroyed = true;
                    }
                }

                for collider in &mut next_colliders {
                    if !collider.is_destroyed && overlaps(&collider.rect, &invader_shot.rect) {
                        collider.is_destroyed = true;
                        invader_shot.is_destroyed = true;
                    }
                }
            }

            player.bullets = next_bullets;
            game.set_all_invader_objects(next_invaders);
            game.invader_shots = next_invader_shots;
            game.set_all_barrier_colliders(next_colliders);

            game.update();
            player.update(&keys);
            ui.update(&keys, &event, &game.state);
        } else {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            for ui_el in &modal_hash {
                canvas.copy(ui_el.1, None, *ui_el.0)?;
            }

            canvas.copy(&arrow_texture, None, ui.get_cursor_target())?;

            for ui_el in &ui_texture_hash {
                canvas.copy(
                    ui_el.1,
                    None,
                    *match ui_targets_hash.get(ui_el.0) {
                        Some(target) => target,
                        None => &default_target,
                    },
                )?;
            }

            canvas.present();
        }
    }

    Ok(())
}
