use crate::game::{
    GameObject, CANVAS_HEIGHT, CANVAS_LEFT_EDGE, CANVAS_RIGHT_EDGE, CANVAS_WIDTH, PIXEL_SIZE,
};

const HEIGHT_DIV_18: u32 = CANVAS_HEIGHT / 18;

pub struct Player {
    pub game_object: GameObject,
    pub bullets: Vec<GameObject>,
    moving_left: bool,
    moving_right: bool,
}

impl Player {
    pub fn new() -> Self {
        Player {
            game_object: GameObject::new(
                CANVAS_WIDTH / 2,
                CANVAS_HEIGHT - HEIGHT_DIV_18,
                15 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("player_texture"),
            ),
            moving_left: false,
            moving_right: false,
            bullets: vec![],
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
            self.game_object.x + (self.game_object.width / 2) - 3,
            self.game_object.y - self.game_object.height / 2,
            1 * PIXEL_SIZE,
            4 * PIXEL_SIZE,
            String::from("shot_texture"),
        ));
    }

    pub fn update(&mut self) {
        if self.moving_left && self.game_object.x > CANVAS_LEFT_EDGE {
            self.game_object.x -= 10;
        }

        if self.moving_right && self.game_object.x < CANVAS_RIGHT_EDGE {
            self.game_object.x += 10;
        }

        if !self.bullets.is_empty() {
            self.bullets.retain(|b| b.y - 10 > 10 && !b.is_destroyed);

            for bullet in &mut self.bullets {
                bullet.y -= 10;
            }
        }
    }
}
