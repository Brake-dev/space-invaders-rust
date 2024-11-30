use crate::game::Game;
use crate::util::overlaps;
use crate::Player;

pub fn update(player: &mut Player, game: &mut Game) {
    'bullets: for bullet in &mut player.bullets {
        for invader in &mut game.invaders {
            if overlaps(&invader.game_object.rect, &bullet.rect) {
                invader.game_object.is_destroyed = true;
                bullet.is_destroyed = true;
                continue 'bullets;
            }
        }

        for barrier in &mut game.barrier_row {
            for collider in &mut barrier.colliders {
                if !collider.is_destroyed && overlaps(&collider.rect, &bullet.rect) {
                    collider.is_destroyed = true;
                    bullet.is_destroyed = true;
                    continue 'bullets;
                }
            }
        }

        if game.ufo_active && overlaps(&game.ufo.game_object.rect, &bullet.rect) {
            game.ufo.game_object.is_destroyed = true;
            bullet.is_destroyed = true;
            continue;
        }
    }

    for invader in &mut game.invaders {
        if overlaps(&invader.game_object.rect, &player.game_object.rect) {
            player.game_object.is_destroyed = true;
        }

        for barrier in &mut game.barrier_row {
            for collider in &mut barrier.colliders {
                if !collider.is_destroyed && overlaps(&collider.rect, &invader.game_object.rect) {
                    collider.is_destroyed = true;
                }
            }
        }
    }

    'invader_shots: for invader_shot in &mut game.invader_shots {
        if overlaps(&invader_shot.rect, &player.game_object.rect) {
            player.game_object.is_destroyed = true;
            continue;
        }

        for bullet in &mut player.bullets {
            if overlaps(&invader_shot.rect, &bullet.rect) {
                invader_shot.is_destroyed = true;
                bullet.is_destroyed = true;
                continue 'invader_shots;
            }
        }

        for barrier in &mut game.barrier_row {
            for collider in &mut barrier.colliders {
                if !collider.is_destroyed && overlaps(&collider.rect, &invader_shot.rect) {
                    collider.is_destroyed = true;
                    invader_shot.is_destroyed = true;
                    continue 'invader_shots;
                }
            }
        }
    }
}
