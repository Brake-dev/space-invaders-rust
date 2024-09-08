use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::game::PIXEL_SIZE;
use crate::texture_templates::{
    BARRIER, INVADER_1, INVADER_2, INVADER_3, INVADER_SHOT, MISSING_TEXTURE, PLAYER, SHOT,
};

pub fn textures<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(HashMap<String, Texture<'a>>, Texture<'a>), String> {
    let mut invader_texture1 = texture_creator
        .create_texture_target(None, INVADER_1[0].len() as u32, INVADER_1.len() as u32)
        .map_err(|e| e.to_string())?;

    let mut invader_texture2 = texture_creator
        .create_texture_target(None, INVADER_2[0].len() as u32, INVADER_2.len() as u32)
        .map_err(|e| e.to_string())?;

    let mut invader_texture3 = texture_creator
        .create_texture_target(None, INVADER_3[0].len() as u32, INVADER_3.len() as u32)
        .map_err(|e| e.to_string())?;

    let mut player_texture = texture_creator
        .create_texture_target(None, PLAYER[0].len() as u32, PLAYER.len() as u32)
        .map_err(|e| e.to_string())?;

    let mut shot_texture = texture_creator
        .create_texture_target(None, 1, SHOT.len() as u32)
        .map_err(|e| e.to_string())?;

    let mut invader_shot_texture = texture_creator
        .create_texture_target(
            None,
            INVADER_SHOT[0].len() as u32,
            INVADER_SHOT.len() as u32,
        )
        .map_err(|e| e.to_string())?;

    let mut barrier_texture = texture_creator
        .create_texture_target(None, BARRIER[0].len() as u32, BARRIER.len() as u32)
        .map_err(|e| e.to_string())?;

    let mut missing_texture = texture_creator
        .create_texture_target(
            None,
            MISSING_TEXTURE.len() as u32,
            MISSING_TEXTURE.len() as u32,
        )
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut invader_texture1, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, _) in INVADER_1.iter().enumerate() {
                for (j, val) in INVADER_1[i].iter().enumerate() {
                    if *val == 0 {
                        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                    } else {
                        texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }

                    texture_canvas
                        .fill_rect(Rect::new(j as i32, i as i32, PIXEL_SIZE, PIXEL_SIZE))
                        .expect("could not draw rect");
                }
            }
        })
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut invader_texture2, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, _) in INVADER_2.iter().enumerate() {
                for (j, val) in INVADER_2[i].iter().enumerate() {
                    if *val == 0 {
                        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                    } else {
                        texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }

                    texture_canvas
                        .fill_rect(Rect::new(j as i32, i as i32, PIXEL_SIZE, PIXEL_SIZE))
                        .expect("could not draw rect");
                }
            }
        })
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut invader_texture3, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, _) in INVADER_3.iter().enumerate() {
                for (j, val) in INVADER_3[i].iter().enumerate() {
                    if *val == 0 {
                        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                    } else {
                        texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }

                    texture_canvas
                        .fill_rect(Rect::new(j as i32, i as i32, PIXEL_SIZE, PIXEL_SIZE))
                        .expect("could not draw rect");
                }
            }
        })
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut player_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, _) in PLAYER.iter().enumerate() {
                for (j, val) in PLAYER[i].iter().enumerate() {
                    if *val == 0 {
                        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                    } else {
                        texture_canvas.set_draw_color(Color::RGB(50, 255, 0));
                    }

                    texture_canvas
                        .fill_rect(Rect::new(j as i32, i as i32, PIXEL_SIZE, PIXEL_SIZE))
                        .expect("could not draw rect");
                }
            }
        })
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut shot_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, _) in SHOT.iter().enumerate() {
                texture_canvas.set_draw_color(Color::RGB(255, 255, 255));

                texture_canvas
                    .fill_rect(Rect::new(0, i as i32, PIXEL_SIZE, PIXEL_SIZE))
                    .expect("could not draw rect");
            }
        })
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut invader_shot_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, _) in INVADER_SHOT.iter().enumerate() {
                for (j, val) in INVADER_SHOT[i].iter().enumerate() {
                    if *val == 0 {
                        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                    } else {
                        texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }

                    texture_canvas
                        .fill_rect(Rect::new(j as i32, i as i32, PIXEL_SIZE, PIXEL_SIZE))
                        .expect("could not draw rect");
                }
            }
        })
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut barrier_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, _) in BARRIER.iter().enumerate() {
                for (j, val) in BARRIER[i].iter().enumerate() {
                    if *val == 0 {
                        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                    } else {
                        texture_canvas.set_draw_color(Color::RGB(50, 255, 0));
                    }

                    texture_canvas
                        .fill_rect(Rect::new(j as i32, i as i32, PIXEL_SIZE, PIXEL_SIZE))
                        .expect("could not draw rect");
                }
            }
        })
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut missing_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, _) in MISSING_TEXTURE.iter().enumerate() {
                for (j, val) in MISSING_TEXTURE[i].iter().enumerate() {
                    if *val == 0 {
                        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                    } else {
                        texture_canvas.set_draw_color(Color::RGB(255, 0, 0));
                    }

                    texture_canvas
                        .fill_rect(Rect::new(j as i32, i as i32, PIXEL_SIZE, PIXEL_SIZE))
                        .expect("could not draw rect");
                }
            }
        })
        .map_err(|e| e.to_string())?;

    let mut hash: HashMap<String, Texture> = HashMap::new();
    hash.insert(String::from("invader_texture1"), invader_texture1);
    hash.insert(String::from("invader_texture2"), invader_texture2);
    hash.insert(String::from("invader_texture3"), invader_texture3);
    hash.insert(String::from("player_texture"), player_texture);
    hash.insert(String::from("shot_texture"), shot_texture);
    hash.insert(String::from("invader_shot_texture"), invader_shot_texture);
    hash.insert(String::from("barrier_texture"), barrier_texture);

    Ok((hash, missing_texture))
}
