pub const PIXEL_SIZE: u32 = 6;
pub const CANVAS_WIDTH: u32 = 1920;
pub const CANVAS_HEIGHT: u32 = 1080;

const ROW_SIZE: u32 = 11;
const ROW_HORIZONTAL_GAP: u32 = 4 * PIXEL_SIZE;
const ROW_VERTICAL_GAP: u32 = 16 * PIXEL_SIZE;

const ROW1_CENTERING_BUFFER: u32 = 16;
const ROW2_3_CENTERING_BUFFER: u32 = PIXEL_SIZE;

const BARRIER_EDGE_BUFFER: u32 = 14 * PIXEL_SIZE;
const BARRIER_GAP: u32 = 88 * PIXEL_SIZE;

const SCREEN_EDGE_BUFFER: u32 = 2 * PIXEL_SIZE;

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
    pub invader_row1: Vec<GameObject>,
    pub invader_row2: Vec<GameObject>,
    pub invader_row3: Vec<GameObject>,
    pub invader_row4: Vec<GameObject>,
    pub invader_row5: Vec<GameObject>,

    pub barrier_row: Vec<GameObject>,
    pub player: Vec<GameObject>,
}

impl Game {
    pub fn new() -> Self {
        let mut invader_row1 = vec![];
        let mut invader_row2 = vec![];
        let mut invader_row3 = vec![];
        let mut invader_row4 = vec![];
        let mut invader_row5 = vec![];

        let mut barrier_row = vec![];

        let mut cur_x = SCREEN_EDGE_BUFFER + ROW1_CENTERING_BUFFER;
        let mut cur_y = CANVAS_HEIGHT / 6;

        for _i in 0..ROW_SIZE {
            invader_row1.push(GameObject::new(
                cur_x,
                cur_y,
                8,
                8,
                String::from("invader_texture1"),
            ));

            cur_x += (8 * PIXEL_SIZE) + ROW_HORIZONTAL_GAP * 2;
        }

        cur_x = SCREEN_EDGE_BUFFER + ROW2_3_CENTERING_BUFFER;
        cur_y += ROW_VERTICAL_GAP;

        for _i in 0..ROW_SIZE {
            invader_row2.push(GameObject::new(
                cur_x,
                cur_y,
                11,
                8,
                String::from("invader_texture2"),
            ));

            cur_x += (11 * PIXEL_SIZE) + ROW_HORIZONTAL_GAP + ROW2_3_CENTERING_BUFFER;
        }

        cur_x = SCREEN_EDGE_BUFFER + ROW2_3_CENTERING_BUFFER;
        cur_y += ROW_VERTICAL_GAP;

        for _i in 0..ROW_SIZE {
            invader_row3.push(GameObject::new(
                cur_x,
                cur_y,
                11,
                8,
                String::from("invader_texture2"),
            ));

            cur_x += (11 * PIXEL_SIZE) + ROW_HORIZONTAL_GAP + ROW2_3_CENTERING_BUFFER;
        }

        cur_x = SCREEN_EDGE_BUFFER + ROW2_3_CENTERING_BUFFER;
        cur_y += ROW_VERTICAL_GAP;

        for _i in 0..ROW_SIZE {
            invader_row4.push(GameObject::new(
                cur_x,
                cur_y,
                12,
                8,
                String::from("invader_texture3"),
            ));

            cur_x += (12 * PIXEL_SIZE) + ROW_HORIZONTAL_GAP;
        }

        cur_x = SCREEN_EDGE_BUFFER;
        cur_y += ROW_VERTICAL_GAP;

        for _i in 0..ROW_SIZE {
            invader_row5.push(GameObject::new(
                cur_x,
                cur_y,
                12,
                8,
                String::from("invader_texture3"),
            ));

            cur_x += (12 * PIXEL_SIZE) + ROW_HORIZONTAL_GAP;
        }

        let mut barrier_x = SCREEN_EDGE_BUFFER + BARRIER_EDGE_BUFFER;

        for _i in 0..4 {
            barrier_row.push(GameObject::new(
                barrier_x,
                CANVAS_HEIGHT - 44 * PIXEL_SIZE,
                24,
                18,
                String::from("barrier_texture"),
            ));

            barrier_x += BARRIER_GAP;
        }

        let player = vec![GameObject::new(
            CANVAS_WIDTH / 2,
            CANVAS_HEIGHT - 10 * PIXEL_SIZE,
            15,
            8,
            String::from("player_texture"),
        )];

        Game {
            invader_row1,
            invader_row2,
            invader_row3,
            invader_row4,
            invader_row5,
            barrier_row,
            player,
        }
    }

    pub fn get_all_textures(&self) -> [&Vec<GameObject>; 7] {
        [
            &self.invader_row1,
            &self.invader_row2,
            &self.invader_row3,
            &self.invader_row4,
            &self.invader_row5,
            &self.barrier_row,
            &self.player,
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
