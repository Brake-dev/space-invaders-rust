use crate::game::{Game, State};
use crate::player::Player;

pub struct Timer {
    pub time: i32,
    pub player_explosion_timer: i32,
    pub game_over_timer: i32,
    pub ufo_timer: u32,
}

impl Timer {
    pub fn new(game: &Game) -> Timer {
        Timer {
            time: 0,
            player_explosion_timer: 0,
            game_over_timer: 0,
            ufo_timer: game.get_next_ufo_time(),
        }
    }

    pub fn update(&mut self, game: &Game, player: &Player) {
        self.time += 1;

        if game.state == State::Playing {
            if player.game_object.is_destroyed {
                self.player_explosion_timer += 1;
                self.game_over_timer += 1;
            }

            if self.ufo_timer == 0 {
                self.ufo_timer = game.get_next_ufo_time();
            } else {
                self.ufo_timer -= 1;
            }
        }
    }
}
