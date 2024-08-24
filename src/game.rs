pub const PIXEL_SIZE: u32 = 9;
pub const CANVAS_WIDTH: u32 = 1920;
pub const CANVAS_HEIGHT: u32 = 1080;

#[derive(Clone, Copy, Debug)]
pub struct GameObject {
    pub x: u32,
    pub y: u32,
}

impl GameObject {
    pub fn new(x: u32, y: u32) -> Self {
        GameObject { x, y }
    }

    fn move_x_right(&mut self) {
        self.x += 1;
    }

    fn move_x_left(&mut self) {
        self.x -= 1;
    }

    fn move_y(&mut self) {
        self.y -= 1;
    }
}

pub struct Game {
    pub game_objects: Vec<GameObject>,
}

impl Game {
    pub fn new(game_objects: Vec<GameObject>) -> Self {
        Game { game_objects }
    }

    pub fn update(&mut self) {
        let mut next_objects = self.game_objects.clone();

        for object in &mut next_objects {
            object.move_x_right();
        }

        self.game_objects = next_objects;
    }
}
