use crate::game::GameObject;
use sdl2::rect::{FRect, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::collections::HashMap;

use crate::game::{CANVAS_HEIGHT, CANVAS_WIDTH};

pub fn draw_texture<'a>(
    canvas: &mut Canvas<Window>,
    textures: &HashMap<String, Texture<'a>>,
    missing_texture: &Texture<'a>,
    object: &GameObject,
) {
    let result = canvas.copy_f(
        match textures.get(&object.texture_name) {
            Some(tex) => tex,
            None => &missing_texture,
        },
        None,
        object.rect,
    );

    match result {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}

pub fn draw_texture_nameless<'a>(canvas: &mut Canvas<Window>, texture: &Texture<'a>, rect: &FRect) {
    let result = canvas.copy_f(texture, None, *rect);

    match result {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}

pub fn draw_texture_nameless_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture: &Texture<'a>,
    rect: &Rect,
) {
    let result = canvas.copy(texture, None, *rect);

    match result {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}

pub fn overlaps(a: &FRect, b: &FRect) -> bool {
    let a_xmax = a.x + a.width();
    let a_ymax = a.y + a.height();

    let b_xmax = b.x + b.width();
    let b_ymax = b.y + b.height();

    a_xmax > b.x && b_xmax > a.x && a_ymax > b.y && b_ymax > a.y
}

pub fn center_x(width: i32) -> i32 {
    CANVAS_WIDTH - width - (width / 2)
}

pub fn center_y(height: i32) -> i32 {
    CANVAS_HEIGHT - height - (height / 2)
}

pub fn decrease_until_zero(value: i32, modifier: i32, min: i32) -> i32 {
    if value - modifier > 0 {
        return value - modifier;
    } else {
        return min;
    }
}
