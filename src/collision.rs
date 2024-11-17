use crate::game::Game;
use crate::util::overlaps;
use crate::Player;

pub fn update(player: &mut Player, game: &mut Game) {
    let mut next_colliders = game.get_all_barrier_colliders();

    for bullet in &mut player.bullets {
        for invader in &mut game.invaders {
            if overlaps(&invader.game_object.rect, &player.game_object.rect) {
                player.game_object.is_destroyed = true;
            }

            if overlaps(&invader.game_object.rect, &bullet.rect) {
                invader.game_object.is_destroyed = true;
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

    for invader_shot in &mut game.invader_shots {
        if overlaps(&invader_shot.rect, &player.game_object.rect) {
            player.game_object.is_destroyed = true;
        }

        for bullet in &mut player.bullets {
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

    game.set_all_barrier_colliders(next_colliders);
}
