use rand::{self, thread_rng, Rng};

use crate::invader::Invader;

pub const PIXEL_SIZE: u32 = 6;
pub const CANVAS_WIDTH: u32 = 1920;
pub const CANVAS_HEIGHT: u32 = 1080;
pub const CANVAS_RIGHT_EDGE: u32 = CANVAS_WIDTH - WIDTH_DIV_20 - 12 * PIXEL_SIZE;
pub const CANVAS_LEFT_EDGE: u32 = WIDTH_DIV_20;

const ROW_SIZE: u32 = 11;

const WIDTH_DIV_4: u32 = CANVAS_WIDTH / 4;
const WIDTH_DIV_20: u32 = CANVAS_WIDTH / 20;
const WIDTH_DIV_24: u32 = CANVAS_WIDTH / 24;
const WIDTH_DIV_80: u32 = CANVAS_WIDTH / 80;
const WIDTH_DIV_240: u32 = CANVAS_WIDTH / 240;
const WIDTH_DIV_320: u32 = CANVAS_WIDTH / 320;

const HEIGHT_DIV_4: u32 = CANVAS_HEIGHT / 4;

const INVADER_SHOT_DELAY: u32 = 100;

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

pub struct Game {
    pub invaders: Vec<Invader>,
    pub barrier_row: Vec<GameObject>,
    pub invader_shots: Vec<GameObject>,
    invader_shot_timer: u32,
}

impl Game {
    pub fn new() -> Self {
        let mut invaders = vec![];

        let mut barrier_row = vec![];

        let mut cur_x = WIDTH_DIV_4;
        let mut cur_y = CANVAS_HEIGHT / 6;

        for i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x,
                cur_y,
                8 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture1"),
                0,
                i,
            ));

            cur_x += (8 * PIXEL_SIZE) + WIDTH_DIV_80 * 2;
        }

        cur_x = WIDTH_DIV_4;
        cur_y += WIDTH_DIV_20;

        for i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x,
                cur_y,
                11 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture2"),
                1,
                i,
            ));

            cur_x += (11 * PIXEL_SIZE) + WIDTH_DIV_80 + WIDTH_DIV_320;
        }

        cur_x = WIDTH_DIV_4 - WIDTH_DIV_240;
        cur_y += WIDTH_DIV_20;

        for i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x,
                cur_y,
                11 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture2"),
                2,
                i,
            ));

            cur_x += (11 * PIXEL_SIZE) + WIDTH_DIV_80 + WIDTH_DIV_320;
        }

        cur_x = WIDTH_DIV_4 - WIDTH_DIV_240;
        cur_y += WIDTH_DIV_20;

        for i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x,
                cur_y,
                12 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture3"),
                3,
                i,
            ));

            cur_x += (12 * PIXEL_SIZE) + WIDTH_DIV_80;
        }

        cur_x = WIDTH_DIV_4 - WIDTH_DIV_240;
        cur_y += WIDTH_DIV_20;

        for i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x,
                cur_y,
                12 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture3"),
                4,
                i,
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
            invader_shots: vec![],
            invader_shot_timer: 0,
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

    pub fn get_last_invader_per_column(&self) -> Vec<i32> {
        let mut columns: Vec<i32> = vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];

        for (i, invader) in self.invaders.iter().enumerate() {
            if columns[invader.column as usize] == -1 || columns[invader.column as usize] < i as i32
            {
                columns[invader.column as usize] = i as i32;
            }
        }

        columns.retain(|i| *i != -1);

        columns
    }

    pub fn get_invader_shooters(&self) -> Vec<i32> {
        let invader_indices = self.get_last_invader_per_column();
        let num = thread_rng().gen_range(1..invader_indices.len());

        let shooters: Vec<i32> = (0..num)
            .map(|_| {
                let index = thread_rng().gen_range(0..invader_indices.len());
                invader_indices[index]
            })
            .collect();

        shooters
    }

    pub fn update(&mut self) {
        let mut invaders_next = self.invaders.clone();

        invaders_next.retain(|r| !r.game_object.is_destroyed);

        let mut move_down = false;
        for invader in &invaders_next {
            if invader.game_object.x == CANVAS_RIGHT_EDGE {
                move_down = true;
                break;
            } else if invader.game_object.x == CANVAS_LEFT_EDGE {
                move_down = true;
                break;
            }
        }

        for invader in &mut invaders_next {
            if move_down {
                invader.move_down();

                if invader.dir == "right" {
                    invader.dir = String::from("left");
                } else {
                    invader.dir = String::from("right");
                }
            }

            if invader.dir == "right" {
                invader.move_x_right();
            } else {
                invader.move_x_left();
            }
        }

        self.invaders = invaders_next;

        self.invader_shot_timer += 1;

        if self.invader_shot_timer >= INVADER_SHOT_DELAY {
            let new_shots = self.get_invader_shooters();

            for shot in new_shots {
                let invader = &self.invaders[shot as usize];

                self.invader_shots.push(GameObject::new(
                    invader.game_object.x + (invader.game_object.width / 2),
                    invader.game_object.y + invader.game_object.height,
                    3 * PIXEL_SIZE,
                    7 * PIXEL_SIZE,
                    String::from("invader_shot_texture"),
                ));
            }

            self.invader_shot_timer = 0;
        }

        let mut invader_shots_next = self.invader_shots.clone();
        invader_shots_next.retain(|s| !s.y > 10 && !s.is_destroyed);

        for shot in &mut invader_shots_next {
            shot.y += 10;
        }

        self.invader_shots = invader_shots_next;
    }
}
