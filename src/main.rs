extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureCreator;

mod barrier;
mod collision;
mod game;
mod invader;
mod player;
mod renderer;
mod texture_templates;
mod textures;
mod ufo;
mod ui;
mod util;

use crate::game::{Game, GameObject, State, CANVAS_HEIGHT, CANVAS_WIDTH, FPS, PIXEL_SIZE};
use crate::player::Player;
use crate::textures::textures;
use crate::ui::{create_ui, UI};

pub struct RetryEvent;
pub struct ContinueEvent;

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
    let (textures, missing_texture, empty_texture) = textures(&mut canvas, &texture_creator)?;
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
    event.register_custom_event::<ContinueEvent>()?;

    let mut time = 0;
    let mut player_explosion_timer = 0;
    let mut game_over_timer = 0;
    let mut ufo_timer = game.get_next_ufo_time();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => game.set_paused(),
                _ => {}
            }

            if event.is_user_event() {
                let retry = event.as_user_event_type::<RetryEvent>();
                let continue_event = event.as_user_event_type::<ContinueEvent>();

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

                match continue_event {
                    Some(_) => game.set_playing(),
                    None => (),
                }
            }
        }

        let keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        fps_manager.delay();

        if game.state == State::Playing {
            if player.game_object.is_destroyed {
                player_explosion_timer += 1;
                game_over_timer += 1;
            }

            if ufo_timer == 0 {
                game.toggle_spawn_ufo();
                ufo_timer = game.get_next_ufo_time();
            } else {
                ufo_timer -= 1;
            }

            if game_over_timer > 1 {
                game.set_game_over();
            }

            renderer::update(
                &mut canvas,
                &game,
                &player,
                &textures,
                &missing_texture,
                &player_explosion_timer,
                &explosion_game_object,
            );

            collision::update(&mut player, &mut game);

            game.update(&time);
            player.update(&keys);
        } else {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            for ui_el in &modal_hash {
                canvas.copy(ui_el.1, None, *ui_el.0)?;
            }

            canvas.copy(&arrow_texture, None, ui.get_cursor_target())?;

            let ui_targets = ui.get_ui_targets_base_on_state(&ui_targets_hash, &game.state);

            for ui_el in ui_targets {
                canvas.copy(
                    match ui_texture_hash.get(&ui_el.0) {
                        Some(texture) => texture,
                        None => &empty_texture,
                    },
                    None,
                    ui_el.1,
                )?;
            }

            canvas.present();
        }

        ui.update(&keys, &event, &game.state);

        time += 1;
    }

    Ok(())
}
