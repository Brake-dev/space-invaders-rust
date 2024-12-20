use std::collections::VecDeque;

use sdl2::rect::FRect;

use rand::{self, thread_rng, Rng};

use crate::barrier::Barrier;
use crate::invader::Invader;
use crate::ufo::UFO;
use crate::util::decrease_until_zero;
use crate::Timer;

pub const FPS: u32 = 60;

pub const PIXEL_SIZE: i32 = 6;
pub const CANVAS_WIDTH: i32 = 1920;
pub const CANVAS_HEIGHT: i32 = 1080;
pub const CANVAS_RIGHT_EDGE: i32 = CANVAS_WIDTH - WIDTH_DIV_20 - 12 * PIXEL_SIZE;
pub const CANVAS_LEFT_EDGE: i32 = WIDTH_DIV_20;

const ROW_SIZE: u32 = 11;

const WIDTH_DIV_4: i32 = CANVAS_WIDTH / 4;
const WIDTH_DIV_20: i32 = CANVAS_WIDTH / 20;
const WIDTH_DIV_24: i32 = CANVAS_WIDTH / 24;
const WIDTH_DIV_80: i32 = CANVAS_WIDTH / 80;
const WIDTH_DIV_240: i32 = CANVAS_WIDTH / 240;
const WIDTH_DIV_320: i32 = CANVAS_WIDTH / 320;

pub const HEIGHT_DIV_4: i32 = CANVAS_HEIGHT / 4;

const INVADER_SHOT_DELAY: u32 = 10;
const EXPLOSION_TIMER: i32 = 2;

const DEFAULT_TICK: i32 = 50;
const TICK_INCREASE: i32 = 12;
const SPEED_INCREASE_LEN: i32 = 15;

#[derive(Debug)]
pub struct GameObject {
    pub rect: FRect,
    pub texture_name: String,
    pub is_destroyed: bool,
}

impl GameObject {
    pub fn new(x: f32, y: f32, width: u32, height: u32, texture_name: String) -> Self {
        GameObject {
            rect: FRect::new(x, y, width as f32, height as f32),
            texture_name,
            is_destroyed: false,
        }
    }
}

pub struct Game {
    pub invaders: Vec<Invader>,
    pub barrier_row: Vec<Barrier>,
    pub invader_shots: Vec<GameObject>,
    loaded_shot: Vec<(i32, i32)>,
    pub explosions: Vec<(GameObject, i32)>,
    invader_shot_timer: u32,
    pub state: State,
    invader_timer: i32,
    speed: i32,
    move_rows_down: VecDeque<u32>,
    pub ufo: UFO,
    spawn_ufo: bool,
    pub ufo_active: bool,
    ufo_spawn_times: u32,
    invader_tick: i32,
    speed_increase_threashold: i32,
}

#[derive(PartialEq)]
pub enum State {
    Playing,
    Paused,
    GameOver,
    Win,
}

impl Game {
    pub fn new() -> Self {
        let mut invaders = vec![];

        let mut barrier_row = vec![];

        let mut cur_x = WIDTH_DIV_4;
        let mut cur_y = CANVAS_HEIGHT / 6;

        for i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x as f32,
                cur_y as f32,
                8 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture1"),
                4,
                i,
            ));

            cur_x += (8 * PIXEL_SIZE) + WIDTH_DIV_80 * 2;
        }

        cur_x = WIDTH_DIV_4;
        cur_y += WIDTH_DIV_20;

        for i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x as f32,
                cur_y as f32,
                11 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture2"),
                3,
                i,
            ));

            cur_x += (11 * PIXEL_SIZE) + WIDTH_DIV_80 + WIDTH_DIV_320;
        }

        cur_x = WIDTH_DIV_4 - WIDTH_DIV_240;
        cur_y += WIDTH_DIV_20;

        for i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x as f32,
                cur_y as f32,
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
                cur_x as f32,
                cur_y as f32,
                12 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture3"),
                1,
                i,
            ));

            cur_x += (12 * PIXEL_SIZE) + WIDTH_DIV_80;
        }

        cur_x = WIDTH_DIV_4 - WIDTH_DIV_240;
        cur_y += WIDTH_DIV_20;

        for i in 0..ROW_SIZE {
            invaders.push(Invader::new(
                cur_x as f32,
                cur_y as f32,
                12 * PIXEL_SIZE,
                8 * PIXEL_SIZE,
                String::from("invader_texture3"),
                0,
                i,
            ));

            cur_x += (12 * PIXEL_SIZE) + WIDTH_DIV_80;
        }

        let mut barrier_x = WIDTH_DIV_24 as f32 * 2.0;

        for _i in 0..4 {
            barrier_row.push(Barrier::new(barrier_x));
            barrier_x += WIDTH_DIV_4 as f32;
        }

        Game {
            invaders,
            barrier_row,
            invader_shots: vec![],
            loaded_shot: vec![],
            explosions: vec![],
            invader_shot_timer: 0,
            state: State::Playing,
            invader_timer: 0,
            speed: 1,
            move_rows_down: VecDeque::new(),
            ufo: UFO::new(0),
            spawn_ufo: false,
            ufo_active: false,
            ufo_spawn_times: 0,
            invader_tick: DEFAULT_TICK,
            speed_increase_threashold: DEFAULT_TICK - SPEED_INCREASE_LEN,
        }
    }

    fn get_last_invader_per_column(&self) -> Vec<i32> {
        let mut columns: Vec<i32> = vec![-1; ROW_SIZE as usize];

        for (i, invader) in self.invaders.iter().enumerate() {
            if columns[invader.column as usize] == -1 || columns[invader.column as usize] < i as i32
            {
                columns[invader.column as usize] = i as i32;
            }
        }

        columns.retain(|i| *i != -1);

        columns
    }

    fn get_invader_shooters(&self) -> Vec<i32> {
        let invader_indices = self.get_last_invader_per_column();

        let mut max = 4;
        if invader_indices.len() < max {
            max = invader_indices.len();
        }

        if max > 1 {
            let num = thread_rng().gen_range(1..max);

            let shooters: Vec<i32> = (0..num)
                .map(|_| {
                    let index = thread_rng().gen_range(0..invader_indices.len());
                    invader_indices[index]
                })
                .collect();

            return shooters;
        }

        vec![]
    }

    pub fn set_playing(&mut self) {
        self.state = State::Playing;
    }

    pub fn set_paused(&mut self) {
        if self.state != State::GameOver {
            self.state = State::Paused;
        }
    }

    pub fn set_game_over(&mut self) {
        self.state = State::GameOver;
    }

    pub fn get_next_ufo_time(&self) -> u32 {
        let adjust: i32 = thread_rng().gen_range(-5..=5);
        let next = 30 + adjust;
        next as u32 * FPS as u32
    }

    pub fn toggle_spawn_ufo(&mut self) {
        if self.spawn_ufo == false {
            self.ufo_spawn_times += 1;
        }

        self.spawn_ufo = !self.spawn_ufo;
    }

    pub fn update(&mut self, timer: &Timer) {
        if timer.ufo_timer == 0 {
            self.toggle_spawn_ufo();
        }

        if timer.game_over_timer > 1 {
            self.set_game_over();
        }

        for invader in &self.invaders {
            if invader.game_object.is_destroyed {
                self.explosions.push((
                    GameObject::new(
                        invader.game_object.rect.x,
                        invader.game_object.rect.y,
                        12 * PIXEL_SIZE as u32,
                        10 * PIXEL_SIZE as u32,
                        String::from("explosion_texture"),
                    ),
                    timer.time + EXPLOSION_TIMER,
                ));
            }
        }

        self.invaders.retain(|r| !r.game_object.is_destroyed);

        if self.invaders.len() as i32 <= self.speed_increase_threashold {
            self.invader_tick = decrease_until_zero(self.invader_tick, TICK_INCREASE, 0);

            self.speed_increase_threashold =
                decrease_until_zero(self.speed_increase_threashold, SPEED_INCREASE_LEN, 1);
        }

        if self.spawn_ufo {
            self.toggle_spawn_ufo();
            self.ufo_active = true;
            self.ufo = UFO::new(self.ufo_spawn_times);
        }

        if self.ufo.game_object.rect.x >= CANVAS_RIGHT_EDGE as f32 && self.ufo.dir == "right"
            || self.ufo.game_object.rect.x <= CANVAS_LEFT_EDGE as f32 && self.ufo.dir == "left"
        {
            self.ufo_active = false;
        }

        if self.ufo_active {
            self.ufo.move_x();
        }

        if self.ufo.game_object.is_destroyed && self.ufo_active == true {
            self.ufo_active = false;

            self.explosions.push((
                GameObject::new(
                    self.ufo.game_object.rect.x,
                    self.ufo.game_object.rect.y,
                    12 * PIXEL_SIZE as u32,
                    10 * PIXEL_SIZE as u32,
                    String::from("explosion_texture"),
                ),
                timer.time + EXPLOSION_TIMER,
            ));
        }

        for invader_shot in &self.invader_shots {
            if invader_shot.is_destroyed {
                self.explosions.push((
                    GameObject::new(
                        invader_shot.rect.x,
                        invader_shot.rect.y,
                        12 * PIXEL_SIZE as u32,
                        10 * PIXEL_SIZE as u32,
                        String::from("explosion_texture"),
                    ),
                    timer.time + EXPLOSION_TIMER,
                ));
            }
        }

        self.explosions.retain(|e| e.1 > timer.time);

        self.invader_shots
            .retain(|s| s.rect.y > 10.0 && !s.is_destroyed);

        for shot in &mut self.invader_shots {
            shot.rect.y += 10.0;
        }

        self.invader_timer += 1 * self.speed;

        if self.invader_timer >= self.invader_tick {
            self.invader_timer = 0;

            let mut move_down = false;
            if self.move_rows_down.len() == 0 {
                for invader in &self.invaders {
                    if invader.game_object.rect.x >= CANVAS_RIGHT_EDGE as f32
                        && invader.dir == "right"
                    {
                        move_down = true;
                        break;
                    } else if invader.game_object.rect.x <= CANVAS_LEFT_EDGE as f32
                        && invader.dir == "left"
                    {
                        move_down = true;
                        break;
                    }
                }
            }

            if move_down {
                for invader in &mut self.invaders {
                    if !self.move_rows_down.contains(&invader.row) {
                        self.move_rows_down.push_front(invader.row);
                    }
                }
            }

            if self.move_rows_down.len() == 0 {
                for invader in &mut self.invaders {
                    if invader.dir == "right" {
                        invader.move_x_right();
                    } else {
                        invader.move_x_left();
                    }
                }
            } else {
                for invader in &mut self.invaders {
                    if invader.row == self.move_rows_down[0] {
                        invader.move_down();

                        if invader.dir == "right" {
                            invader.dir = String::from("left");
                        } else {
                            invader.dir = String::from("right");
                        }
                    }
                }

                self.move_rows_down.pop_front();
            }

            self.invader_shot_timer += 1;

            if self.invader_shot_timer >= INVADER_SHOT_DELAY {
                let new_shots = self.get_invader_shooters();

                for (i, shot) in new_shots.iter().enumerate() {
                    self.loaded_shot
                        .push((*shot, timer.time + i as i32 * self.invader_tick));
                }

                self.invader_shot_timer = 0;
            }

            for shot in &self.loaded_shot {
                if shot.1 <= timer.time {
                    if self.invaders.len() > shot.0 as usize {
                        let invader = &self.invaders[shot.0 as usize];

                        if !invader.game_object.is_destroyed {
                            self.invader_shots.push(GameObject::new(
                                invader.game_object.rect.x
                                    + (invader.game_object.rect.width() / 2.0),
                                invader.game_object.rect.y + invader.game_object.rect.height(),
                                3 * PIXEL_SIZE as u32,
                                7 * PIXEL_SIZE as u32,
                                String::from("invader_shot_texture"),
                            ));
                        }
                    }
                }
            }

            self.loaded_shot.retain(|s| s.1 > timer.time);
        }
    }
}
