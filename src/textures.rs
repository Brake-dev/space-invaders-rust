use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::barrier::Collider;
use crate::game::PIXEL_SIZE;
use crate::texture_templates::{
    BARRIER, BARRIER_MASK, EXPLOSION, INVADER_1_1, INVADER_1_2, INVADER_2_1, INVADER_2_2,
    INVADER_3_1, INVADER_3_2, INVADER_SHOT, MISSING_TEXTURE, PLAYER, SHOT, UFO,
};

fn get_texture_from_matrix<'a, const T: usize, const M: usize>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    matrix: [[u32; T]; M],
    color: Color,
    format: Option<PixelFormatEnum>,
) -> Result<Texture<'a>, String> {
    let row_lenght = matrix[0].len();

    let mut texture = texture_creator
        .create_texture_target(format, row_lenght as u32, matrix.len() as u32)
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, row) in matrix.iter().enumerate() {
                for (j, val) in row.iter().enumerate() {
                    if *val == 0 {
                        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                    } else {
                        texture_canvas.set_draw_color(color);
                    }

                    texture_canvas
                        .fill_rect(Rect::new(
                            j as i32,
                            i as i32,
                            PIXEL_SIZE as u32,
                            PIXEL_SIZE as u32,
                        ))
                        .expect("could not draw rect");
                }
            }
        })
        .map_err(|e| e.to_string())?;

    Ok(texture)
}

pub fn textures<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(HashMap<String, Texture<'a>>, Texture<'a>, Texture<'a>), String> {
    let invader_texture1_1 = get_texture_from_matrix(
        canvas,
        texture_creator,
        INVADER_1_1,
        Color::RGB(255, 255, 255),
        None,
    )?;

    let invader_texture1_2 = get_texture_from_matrix(
        canvas,
        texture_creator,
        INVADER_1_2,
        Color::RGB(255, 255, 255),
        None,
    )?;

    let invader_texture2_1 = get_texture_from_matrix(
        canvas,
        texture_creator,
        INVADER_2_1,
        Color::RGB(255, 255, 255),
        None,
    )?;

    let invader_texture2_2 = get_texture_from_matrix(
        canvas,
        texture_creator,
        INVADER_2_2,
        Color::RGB(255, 255, 255),
        None,
    )?;

    let invader_texture3_1 = get_texture_from_matrix(
        canvas,
        texture_creator,
        INVADER_3_1,
        Color::RGB(255, 255, 255),
        None,
    )?;

    let invader_texture3_2 = get_texture_from_matrix(
        canvas,
        texture_creator,
        INVADER_3_2,
        Color::RGB(255, 255, 255),
        None,
    )?;

    let player_texture = get_texture_from_matrix(
        canvas,
        texture_creator,
        PLAYER,
        Color::RGB(50, 255, 0),
        None,
    )?;

    let explosion_texture = get_texture_from_matrix(
        canvas,
        texture_creator,
        EXPLOSION,
        Color::RGB(255, 255, 255),
        None,
    )?;

    let invader_shot_texture = get_texture_from_matrix(
        canvas,
        texture_creator,
        INVADER_SHOT,
        Color::RGB(255, 255, 255),
        None,
    )?;

    let barrier_texture = get_texture_from_matrix(
        canvas,
        texture_creator,
        BARRIER,
        Color::RGB(50, 255, 0),
        None,
    )?;

    let mut barrier_mask_texture = get_texture_from_matrix(
        canvas,
        texture_creator,
        BARRIER_MASK,
        Color::RGB(0, 0, 0),
        Some(PixelFormatEnum::ABGR8888),
    )?;

    barrier_mask_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

    let ufo_texture =
        get_texture_from_matrix(canvas, texture_creator, UFO, Color::RGB(255, 0, 0), None)?;

    let mut shot_texture = texture_creator
        .create_texture_target(None, 1, SHOT.len() as u32)
        .map_err(|e| e.to_string())?;

    let missing_texture = get_texture_from_matrix(
        canvas,
        texture_creator,
        MISSING_TEXTURE,
        Color::RGB(255, 0, 0),
        None,
    )?;

    let mut empty_texture = texture_creator
        .create_texture_target(None, 1, 1)
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut shot_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, _) in SHOT.iter().enumerate() {
                texture_canvas.set_draw_color(Color::RGB(255, 255, 255));

                texture_canvas
                    .fill_rect(Rect::new(0, i as i32, PIXEL_SIZE as u32, PIXEL_SIZE as u32))
                    .expect("could not draw rect");
            }
        })
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut empty_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            texture_canvas
                .fill_rect(Rect::new(0, 0, 0, 0))
                .expect("could not draw rect");
        })
        .map_err(|e| e.to_string())?;

    let mut hash: HashMap<String, Texture> = HashMap::new();
    hash.insert(String::from("invader_texture1_1"), invader_texture1_1);
    hash.insert(String::from("invader_texture1_2"), invader_texture1_2);
    hash.insert(String::from("invader_texture2_1"), invader_texture2_1);
    hash.insert(String::from("invader_texture2_2"), invader_texture2_2);
    hash.insert(String::from("invader_texture3_1"), invader_texture3_1);
    hash.insert(String::from("invader_texture3_2"), invader_texture3_2);
    hash.insert(String::from("player_texture"), player_texture);
    hash.insert(String::from("shot_texture"), shot_texture);
    hash.insert(String::from("explosion_texture"), explosion_texture);
    hash.insert(String::from("invader_shot_texture"), invader_shot_texture);
    hash.insert(String::from("barrier_texture"), barrier_texture);
    hash.insert(String::from("barrier_mask_texture"), barrier_mask_texture);
    hash.insert(String::from("ufo_texture"), ufo_texture);

    Ok((hash, missing_texture, empty_texture))
}

#[allow(dead_code)]
pub fn get_collider_textures<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    colliders: &Vec<Collider>,
) -> Result<Vec<Texture<'a>>, String> {
    let mut targets = vec![];

    for collider in colliders {
        let target = texture_creator
            .create_texture_target(
                None,
                collider.rect.width() as u32,
                collider.rect.height() as u32,
            )
            .map_err(|e| e.to_string())?;

        targets.push((target, collider));
    }

    let mut targets_for_canvas = vec![];
    for target in &mut targets {
        targets_for_canvas.push((&mut target.0, target.1));
    }

    canvas
        .with_multiple_texture_canvas(targets_for_canvas.iter(), |texture_canvas, collider| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for item in 0..=collider.rect.y() as i32 {
                texture_canvas.set_draw_color(Color::RGB(255, 0, 0));

                texture_canvas
                    .fill_rect(Rect::new(0, item, PIXEL_SIZE as u32, PIXEL_SIZE as u32))
                    .expect("could not draw rect");
            }
        })
        .map_err(|e| e.to_string())?;

    let mut textures = vec![];
    for target in targets {
        textures.push(target.0);
    }

    Ok(textures)
}
