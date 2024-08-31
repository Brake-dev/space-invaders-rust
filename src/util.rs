use crate::game::GameObject;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::collections::HashMap;

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
        Rect::new(
            object.x as i32,
            object.y as i32,
            object.width,
            object.height,
        ),
    )
}

pub fn overlaps(a: &GameObject, b: &GameObject) {
    let a_xmax = a.x + a.width;
    let a_ymax = a.y + a.height;

    let b_xmax = b.x + b.width;
    let b_ymax = b.y + b.height;

    if a_xmax > b.x && b_xmax > a.x && a_ymax > b.y && b_ymax > a.y {
        println!("Collide!");
    }
}
