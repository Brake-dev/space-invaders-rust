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
}

impl GameObject {
    pub fn new(x: u32, y: u32, width: u32, height: u32, texture_name: String) -> Self {
        GameObject {
            x,
            y,
            width,
            height,
            texture_name,
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
                15,
                8,
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
            self.game_object.x + ((self.game_object.width * PIXEL_SIZE) / 2) - 3,
            self.game_object.y - (self.game_object.height * PIXEL_SIZE) / 2,
            1,
            4,
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

            next_bullets.retain(|b| b.y - 10 > 10);

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
}

impl Invader {
    pub fn new(x: u32, y: u32, width: u32, height: u32, texture_name: String) -> Self {
        Invader {
            game_object: GameObject::new(x, y, width, height, texture_name),
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
    pub invader_row1: Vec<Invader>,
    pub invader_row2: Vec<Invader>,
    pub invader_row3: Vec<Invader>,
    pub invader_row4: Vec<Invader>,
    pub invader_row5: Vec<Invader>,
    pub barrier_row: Vec<GameObject>,
}

impl Game {
    pub fn new() -> Self {
        let mut invader_row1 = vec![];
        let mut invader_row2 = vec![];
        let mut invader_row3 = vec![];
        let mut invader_row4 = vec![];
        let mut invader_row5 = vec![];

        let mut barrier_row = vec![];

        let mut cur_x = WIDTH_DIV_80;
        let mut cur_y = CANVAS_HEIGHT / 6;

        for _i in 0..ROW_SIZE {
            invader_row1.push(Invader::new(
                cur_x,
                cur_y,
                8,
                8,
                String::from("invader_texture1"),
            ));

            cur_x += (8 * PIXEL_SIZE) + WIDTH_DIV_80 * 2;
        }

        cur_x = WIDTH_DIV_160 + WIDTH_DIV_320;
        cur_y += WIDTH_DIV_20;

        for _i in 0..ROW_SIZE {
            invader_row2.push(Invader::new(
                cur_x,
                cur_y,
                11,
                8,
                String::from("invader_texture2"),
            ));

            cur_x += (11 * PIXEL_SIZE) + WIDTH_DIV_80 + WIDTH_DIV_320;
        }

        cur_x = WIDTH_DIV_160 + WIDTH_DIV_320;
        cur_y += WIDTH_DIV_20;

        for _i in 0..ROW_SIZE {
            invader_row3.push(Invader::new(
                cur_x,
                cur_y,
                11,
                8,
                String::from("invader_texture2"),
            ));

            cur_x += (11 * PIXEL_SIZE) + WIDTH_DIV_80 + WIDTH_DIV_320;
        }

        cur_x = WIDTH_DIV_160 + WIDTH_DIV_320;
        cur_y += WIDTH_DIV_20;

        for _i in 0..ROW_SIZE {
            invader_row4.push(Invader::new(
                cur_x,
                cur_y,
                12,
                8,
                String::from("invader_texture3"),
            ));

            cur_x += (12 * PIXEL_SIZE) + WIDTH_DIV_80;
        }

        cur_x = WIDTH_DIV_160;
        cur_y += WIDTH_DIV_20;

        for _i in 0..ROW_SIZE {
            invader_row5.push(Invader::new(
                cur_x,
                cur_y,
                12,
                8,
                String::from("invader_texture3"),
            ));

            cur_x += (12 * PIXEL_SIZE) + WIDTH_DIV_80;
        }

        let mut barrier_x = WIDTH_DIV_24 * 2;

        for _i in 0..4 {
            barrier_row.push(GameObject::new(
                barrier_x,
                CANVAS_HEIGHT - HEIGHT_DIV_4,
                24,
                18,
                String::from("barrier_texture"),
            ));

            barrier_x += WIDTH_DIV_4;
        }

        Game {
            invader_row1,
            invader_row2,
            invader_row3,
            invader_row4,
            invader_row5,
            barrier_row,
        }
    }

    pub fn get_invader_barrier_textures(&self) -> [Vec<GameObject>; 6] {
        let invader_row1_game_object: Vec<GameObject> = self
            .invader_row1
            .iter()
            .map(|i| i.game_object.clone())
            .collect();

        let invader_row2_game_object: Vec<GameObject> = self
            .invader_row2
            .iter()
            .map(|i| i.game_object.clone())
            .collect();

        let invader_row3_game_object: Vec<GameObject> = self
            .invader_row3
            .iter()
            .map(|i| i.game_object.clone())
            .collect();

        let invader_row4_game_object: Vec<GameObject> = self
            .invader_row4
            .iter()
            .map(|i| i.game_object.clone())
            .collect();

        let invader_row5_game_object: Vec<GameObject> = self
            .invader_row5
            .iter()
            .map(|i| i.game_object.clone())
            .collect();

        let barrier_game_object = self.barrier_row.clone();

        [
            invader_row1_game_object,
            invader_row2_game_object,
            invader_row3_game_object,
            invader_row4_game_object,
            invader_row5_game_object,
            barrier_game_object,
        ]
    }

    pub fn update(&mut self) {
        let mut invader_row1_next = self.invader_row1.clone();
        let mut invader_row2_next = self.invader_row2.clone();
        let mut invader_row3_next = self.invader_row3.clone();
        let mut invader_row4_next = self.invader_row4.clone();
        let mut invader_row5_next = self.invader_row5.clone();

        for object in &mut invader_row1_next {
            object.move_x_right();
        }

        for object in &mut invader_row2_next {
            object.move_x_right();
        }

        for object in &mut invader_row3_next {
            object.move_x_right();
        }

        for object in &mut invader_row4_next {
            object.move_x_right();
        }

        for object in &mut invader_row5_next {
            object.move_x_right();
        }

        self.invader_row1 = invader_row1_next;
        self.invader_row2 = invader_row2_next;
        self.invader_row3 = invader_row3_next;
        self.invader_row4 = invader_row4_next;
        self.invader_row5 = invader_row5_next;
    }
}
