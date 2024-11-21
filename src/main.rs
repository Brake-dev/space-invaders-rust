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
mod sdl_common;
mod texture_templates;
mod textures;
mod timer;
mod ufo;
mod ui;
mod util;

use crate::game::{Game, GameObject, State, FPS, PIXEL_SIZE};
use crate::player::Player;
use crate::sdl_common::{ContinueEvent, RetryEvent};
use crate::textures::textures;
use crate::timer::Timer;
use crate::ui::{create_ui, UI};

fn main() -> Result<(), String> {
    let (mut canvas, event, mut event_pump) = sdl_common::init()?;

    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(FPS)?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut game = Game::new();
    let mut player = Player::new();
    let mut timer = Timer::new(&game);

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
                        timer = Timer::new(&game);
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
            renderer::update(
                &mut canvas,
                &game,
                &player,
                &textures,
                &missing_texture,
                &timer.player_explosion_timer,
                &explosion_game_object,
            );

            collision::update(&mut player, &mut game);

            game.update(&timer);
            player.update(&keys);
        } else {
            renderer::update_ui(
                &mut canvas,
                &modal_hash,
                &arrow_texture,
                &ui,
                &game,
                &ui_targets_hash,
                &ui_texture_hash,
                &empty_texture,
            )
        }

        ui.update(&keys, &event, &game.state);

        timer.update(&game, &player);
    }

    Ok(())
}
