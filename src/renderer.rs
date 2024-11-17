use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::game::{Game, GameObject, PIXEL_SIZE};
use crate::player::Player;
use crate::util::{draw_texture, draw_texture_nameless};

pub fn update<'a>(
    canvas: &mut Canvas<Window>,
    game: &Game,
    player: &Player,
    textures: &HashMap<String, Texture<'a>>,
    missing_texture: &Texture<'a>,
    player_explosion_timer: &i32,
    explosion_game_object: &GameObject,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for invader in &game.invaders {
        draw_texture(canvas, textures, missing_texture, &invader.game_object);
    }

    for shot in &game.invader_shots {
        draw_texture(canvas, textures, missing_texture, &shot);
    }

    for explosion in &game.explosions {
        draw_texture(canvas, textures, missing_texture, &explosion.0);
    }

    for barrier in &game.barrier_row {
        draw_texture(canvas, textures, missing_texture, &barrier.game_object);

        for collider in &barrier.colliders {
            if collider.is_destroyed {
                draw_texture_nameless(
                    canvas,
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
                );
            }
        }
    }

    if !player.game_object.is_destroyed {
        draw_texture(canvas, textures, missing_texture, &player.game_object);
    }

    if *player_explosion_timer > 0 && *player_explosion_timer < 10 {
        draw_texture(canvas, textures, missing_texture, &explosion_game_object);
    }

    if game.ufo_active {
        draw_texture(canvas, textures, missing_texture, &game.ufo.game_object);
    }

    for bullet in &player.bullets {
        draw_texture(canvas, textures, missing_texture, &bullet);
    }

    canvas.present();
}
