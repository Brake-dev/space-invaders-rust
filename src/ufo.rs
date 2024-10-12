use crate::game::{GameObject, CANVAS_LEFT_EDGE, CANVAS_RIGHT_EDGE, PIXEL_SIZE};

pub struct UFO {
    pub game_object: GameObject,
    pub dir: String,
}

impl UFO {
    pub fn new(spawn_times: u32) -> UFO {
        let width = 16 * PIXEL_SIZE;
        let height = 7 * PIXEL_SIZE;

        let mut x = CANVAS_LEFT_EDGE;
        let mut dir = String::from("right");

        if spawn_times % 2 != 0 {
            x = CANVAS_RIGHT_EDGE;
            dir = String::from("left")
        }

        UFO {
            game_object: GameObject::new(x, height, width, height, String::from("ufo_texture")),
            dir,
        }
    }

    pub fn move_x(&mut self) {
        if self.dir == "right" {
            self.game_object.x += 10;
        } else {
            self.game_object.x -= 10;
        }
    }
}
