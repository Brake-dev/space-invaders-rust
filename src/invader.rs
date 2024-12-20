use crate::game::{GameObject, PIXEL_SIZE};

#[derive(Debug)]
pub struct Invader {
    pub game_object: GameObject,
    pub row: u32,
    pub column: u32,
    pub dir: String,
    pub anim_step: i32,
}

impl Invader {
    pub fn new(
        x: f32,
        y: f32,
        width: i32,
        height: i32,
        texture_name: String,
        row: u32,
        column: u32,
    ) -> Self {
        Invader {
            game_object: GameObject::new(x, y, width as u32, height as u32, texture_name),
            row,
            column,
            dir: String::from("right"),
            anim_step: 1,
        }
    }

    pub fn move_x_right(&mut self) {
        self.game_object.rect.x += 10.0;
        self.toggle_anim_step();
    }

    pub fn move_x_left(&mut self) {
        self.game_object.rect.x -= 10.0;
        self.toggle_anim_step();
    }

    pub fn move_down(&mut self) {
        self.game_object.rect.y += 8.0 * PIXEL_SIZE as f32;
        self.anim_step = 1;
    }

    fn toggle_anim_step(&mut self) {
        self.anim_step = if self.anim_step == 1 { 2 } else { 1 }
    }
}
