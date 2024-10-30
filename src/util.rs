use crate::game::GameObject;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::collections::HashMap;

use crate::game::{CANVAS_HEIGHT, CANVAS_WIDTH};

pub fn draw_texture<'a>(
    canvas: &mut Canvas<Window>,
    textures: &HashMap<String, Texture<'a>>,
    missing_texture: &Texture<'a>,
    object: &GameObject,
) -> Result<(), String> {
    canvas.copy(
        match textures.get(&object.texture_name) {
            Some(tex) => tex,
            None => &missing_texture,
        },
        None,
        object.rect,
    )
}

pub fn draw_texture_nameless<'a>(
    canvas: &mut Canvas<Window>,
    texture: &Texture<'a>,
    rect: &Rect,
) -> Result<(), String> {
    canvas.copy(texture, None, *rect)
}

pub fn overlaps(a: &Rect, b: &Rect) -> bool {
    let a_xmax = a.x + a.width() as i32;
    let a_ymax = a.y + a.height() as i32;

    let b_xmax = b.x + b.width() as i32;
    let b_ymax = b.y + b.height() as i32;

    a_xmax > b.x && b_xmax > a.x && a_ymax > b.y && b_ymax > a.y
}

pub fn center_x(width: i32) -> i32 {
    CANVAS_WIDTH - width - (width / 2)
}

pub fn center_y(height: i32) -> i32 {
    CANVAS_HEIGHT - height - (height / 2)
}
