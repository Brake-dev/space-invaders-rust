use sdl2::rect::Rect;

use crate::game::{GameObject, CANVAS_HEIGHT, HEIGHT_DIV_4, PIXEL_SIZE};
use crate::texture_templates::BARRIER;

#[derive(Clone, Debug)]
pub struct Collider {
    pub rect: Rect,
    pub is_destroyed: bool,
}

pub struct Barrier {
    pub game_object: GameObject,
    pub colliders: Vec<Collider>,
}

impl Barrier {
    pub fn new(x: u32) -> Barrier {
        let y = CANVAS_HEIGHT - HEIGHT_DIV_4;
        let width = 24 * PIXEL_SIZE;
        let height = 18 * PIXEL_SIZE;

        let game_object = GameObject::new(x, y, width, height, String::from("barrier_texture"));
        let colliders = Collider::get_colliders(&game_object);

        Barrier {
            game_object,
            colliders,
        }
    }
}

impl Collider {
    pub fn new(x: u32, y: u32, height: u32) -> Collider {
        Collider {
            rect: Rect::new(x as i32, y as i32, PIXEL_SIZE, height * PIXEL_SIZE),
            is_destroyed: false,
        }
    }

    pub fn get_colliders(barrier: &GameObject) -> Vec<Collider> {
        let mut colliders = vec![];

        let mut next_x;
        let mut next_y;

        let collider_heights_1 = [
            [5, 5, 5],
            [5, 5, 6],
            [5, 6, 6],
            [6, 6, 6],
            [6, 6, 6],
            [6, 6, 6],
            [6, 6, 0],
            [6, 5, 0],
            [5, 5, 0],
            [4, 5, 0],
            [4, 4, 0],
            [4, 4, 0],
        ];

        let mut collider_heights_2 = collider_heights_1.clone();
        collider_heights_2.reverse();

        let collider_heights = [collider_heights_1, collider_heights_2].concat();

        for (i, _) in BARRIER[0].iter().enumerate() {
            if i == 0 || i == 23 {
                next_y = barrier.y + (3 * PIXEL_SIZE);
            } else if i == 1 || i == 22 {
                next_y = barrier.y + (2 * PIXEL_SIZE);
            } else if i == 2 || i == 21 {
                next_y = barrier.y + PIXEL_SIZE;
            } else {
                next_y = barrier.y;
            }

            next_x = barrier.x + (i as u32 * PIXEL_SIZE);

            for height in collider_heights[i] {
                if height > 0 {
                    colliders.push(Collider::new(next_x, next_y, height));
                    next_y = next_y + (height * PIXEL_SIZE);
                }
            }
        }

        colliders
    }
}
