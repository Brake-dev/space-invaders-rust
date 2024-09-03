use crate::game::{GameObject, PIXEL_SIZE};

#[derive(Clone)]
pub struct Invader {
    pub game_object: GameObject,
    pub row: u32,
    pub column: u32,
    pub dir: String,
}

impl Invader {
    pub fn new(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        texture_name: String,
        row: u32,
        column: u32,
    ) -> Self {
        Invader {
            game_object: GameObject::new(x, y, width, height, texture_name),
            row,
            column,
            dir: String::from("right"),
        }
    }

    pub fn move_x_right(&mut self) {
        self.game_object.x += 1;
    }

    pub fn move_x_left(&mut self) {
        self.game_object.x -= 1;
    }

    pub fn move_down(&mut self) {
        self.game_object.y += 8 * PIXEL_SIZE;
    }
}
