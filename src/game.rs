pub const PIXEL_SIZE: u32 = 6;
pub const CANVAS_WIDTH: u32 = 1920;
pub const CANVAS_HEIGHT: u32 = 1080;

const ROW_SIZE: u32 = 11;

const WIDTH_DIV_4: u32 = CANVAS_WIDTH / 4;
const WIDTH_DIV_20: u32 = CANVAS_WIDTH / 20;
const WIDTH_DIV_24: u32 = CANVAS_WIDTH / 24;
const WIDTH_DIV_80: u32 = CANVAS_WIDTH / 80;
const WIDTH_DIV_160: u32 = CANVAS_WIDTH / 160;
const WIDTH_DIV_320: u32 = CANVAS_WIDTH / 320;

const HEIGHT_DIV_4: u32 = CANVAS_HEIGHT / 4;
const HEIGHT_DIV_18: u32 = CANVAS_HEIGHT / 18;

#[derive(Clone, Debug)]
pub struct GameObject {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub texture_name: String,
    pub is_destroyed: bool,
}

impl GameObject {
    pub fn new(x: u32, y: u32, width: u32, height: u32, texture_name: String) -> Self {
        GameObject {
            x,
            y,
            width,
            height,
            texture_name,
            is_destroyed: false,
        }
    }
}

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
        if self.moving_left {
            self.game_object.x -= 10;
        }

        if self.moving_right {
            self.game_object.x += 10;
        }

        if !self.bullets.is_empty() {
            let mut next_bullets = self.bullets.clone();

            next_bullets.retain(|b| b.y - 10 > 10 && !b.is_destroyed);

            for bullet in &mut next_bullets {
                bullet.y -= 10;
            }

            self.bullets = next_bullets;
        }
    }
}

#[derive(Clone)]
pub struct Invader {
    pub game_object: GameObject,
    pub row: u32,
}

impl Invader {
    pub fn new(x: u32, y: u32, width: u32, height: u32, texture_name: String, row: u32) -> Self {
        Invader {
            game_object: GameObject::new(x, y, width, height, texture_name),
            row,
        }
    }

    fn move_x_right(&mut self) {
        self.game_object.x += 1;
    }

    fn move_x_left(&mut self) {
        self.game_object.x -= 1;
    }

    fn move_y(&mut self) {
        self.game_object.y -= 1;
    }
}

pub struct Game {
    pub invaders: Vec<Invader>,
    pub barrier_row: Vec<GameObject>,
}

impl Game {
    pub fn new() -> Self {
        let mut invaders = vec![];

        let mut barrier_row = vec![];

        let mut cur_x = WIDTH_DIV_80;
        let mut cur_y = CANVAS_HEIGHT / 6;

        for _i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x,
                cur_y,
                8 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture1"),
                1,
            ));

            cur_x += (8 * PIXEL_SIZE) + WIDTH_DIV_80 * 2;
        }

        cur_x = WIDTH_DIV_160 + WIDTH_DIV_320;
        cur_y += WIDTH_DIV_20;

        for _i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x,
                cur_y,
                11 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture2"),
                2,
            ));

            cur_x += (11 * PIXEL_SIZE) + WIDTH_DIV_80 + WIDTH_DIV_320;
        }

        cur_x = WIDTH_DIV_160 + WIDTH_DIV_320;
        cur_y += WIDTH_DIV_20;

        for _i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x,
                cur_y,
                11 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture2"),
                3,
            ));

            cur_x += (11 * PIXEL_SIZE) + WIDTH_DIV_80 + WIDTH_DIV_320;
        }

        cur_x = WIDTH_DIV_160 + WIDTH_DIV_320;
        cur_y += WIDTH_DIV_20;

        for _i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x,
                cur_y,
                12 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture3"),
                4,
            ));

            cur_x += (12 * PIXEL_SIZE) + WIDTH_DIV_80;
        }

        cur_x = WIDTH_DIV_160;
        cur_y += WIDTH_DIV_20;

        for _i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x,
                cur_y,
                12 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture3"),
                5,
            ));

            cur_x += (12 * PIXEL_SIZE) + WIDTH_DIV_80;
        }

        let mut barrier_x = WIDTH_DIV_24 * 2;

        for _i in 0..4 {
            barrier_row.push(GameObject::new(
                barrier_x,
                CANVAS_HEIGHT - HEIGHT_DIV_4,
                24 * PIXEL_SIZE,
                18 * PIXEL_SIZE,
                String::from("barrier_texture"),
            ));

            barrier_x += WIDTH_DIV_4;
        }

        Game {
            invaders,
            barrier_row,
        }
    }

    pub fn set_all_invader_objects(&mut self, next_objects: Vec<GameObject>) {
        for (i, invader) in &mut self.invaders.iter_mut().enumerate() {
            invader.game_object = next_objects[i].clone();
        }
    }

    pub fn get_all_invader_objects(&self) -> Vec<GameObject> {
        self.invaders
            .iter()
            .map(|i| i.game_object.clone())
            .collect()
    }

    pub fn update(&mut self) {
        let mut invaders_next = self.invaders.clone();

        invaders_next.retain(|r| !r.game_object.is_destroyed);

        for object in &mut invaders_next {
            object.move_x_right();
        }

        self.invaders = invaders_next;
    }
}
