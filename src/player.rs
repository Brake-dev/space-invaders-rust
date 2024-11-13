use std::collections::HashSet;

use sdl2::keyboard::Keycode;

use crate::game::{
    GameObject, CANVAS_HEIGHT, CANVAS_LEFT_EDGE, CANVAS_RIGHT_EDGE, CANVAS_WIDTH, PIXEL_SIZE,
};

const HEIGHT_DIV_18: i32 = CANVAS_HEIGHT / 18;

pub struct Player {
    pub game_object: GameObject,
    pub bullets: Vec<GameObject>,
    moving_left: bool,
    moving_right: bool,
    prev_keys: HashSet<Keycode>,
    shot_timer: u32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            game_object: GameObject::new(
                CANVAS_WIDTH / 2,
                CANVAS_HEIGHT - HEIGHT_DIV_18,
                15 * PIXEL_SIZE as u32,
                8 * PIXEL_SIZE as u32,
                String::from("player_texture"),
            ),
            moving_left: false,
            moving_right: false,
            bullets: vec![],
            prev_keys: HashSet::new(),
            shot_timer: 1,
        }
    }

    pub fn set_moving_left(&mut self, moving: bool) {
        self.moving_left = moving;
        if moving {
            self.moving_right = false;
        }
    }

    pub fn set_moving_right(&mut self, moving: bool) {
        self.moving_right = moving;
        if moving {
            self.moving_left = false;
        }
    }

    pub fn shoot(&mut self) {
        self.bullets.push(GameObject::new(
            self.game_object.rect.x + (self.game_object.rect.width() / 2) as i32 - 3,
            self.game_object.rect.y - (self.game_object.rect.height() / 2) as i32,
            1 * PIXEL_SIZE as u32,
            4 * PIXEL_SIZE as u32,
            String::from("shot_texture"),
        ));
    }

    pub fn update(&mut self, keys: &HashSet<Keycode>) {
        if self.game_object.is_destroyed {
            return;
        }

        let new_keys = keys - &self.prev_keys;
        let old_keys = &self.prev_keys - keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            if new_keys.contains(&Keycode::Left) {
                self.set_moving_left(true);
            } else if old_keys.contains(&Keycode::Left) {
                self.set_moving_left(false);
            }

            if new_keys.contains(&Keycode::Right) {
                self.set_moving_right(true);
            } else if old_keys.contains(&Keycode::Right) {
                self.set_moving_right(false);
            }

            if new_keys.contains(&Keycode::Space) && self.shot_timer == 0 {
                self.shoot();
                self.shot_timer = 20;
            }
        }

        self.prev_keys = keys.clone();

        if self.moving_left && self.game_object.rect.x > CANVAS_LEFT_EDGE {
            self.game_object.rect.x -= 10;
        }

        if self.moving_right && self.game_object.rect.x < CANVAS_RIGHT_EDGE {
            self.game_object.rect.x += 10;
        }

        if !self.bullets.is_empty() {
            self.bullets
                .retain(|b| b.rect.y - 10 > 10 && !b.is_destroyed);

            for bullet in &mut self.bullets {
                bullet.rect.y -= 10;
            }
        }

        if self.shot_timer > 0 {
            self.shot_timer -= 1;
        }
    }
}
