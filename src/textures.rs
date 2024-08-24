use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::game::PIXEL_SIZE;

pub enum TextureColor {
    Black,
    White,
    Green,
    Red,
}

pub fn textures<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(Texture<'a>, Texture<'a>), String> {
    let mut square_texture1 = texture_creator
        .create_texture_target(None, PIXEL_SIZE, PIXEL_SIZE)
        .map_err(|e| e.to_string())?;

    let mut square_texture2 = texture_creator
        .create_texture_target(None, PIXEL_SIZE, PIXEL_SIZE)
        .map_err(|e| e.to_string())?;

    let textures = [
        (&mut square_texture1, TextureColor::White),
        (&mut square_texture2, TextureColor::Green),
    ];

    canvas
        .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            match *user_context {
                TextureColor::White => {
                    for i in 0..PIXEL_SIZE {
                        for j in 0..PIXEL_SIZE {
                            texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                            texture_canvas
                                .draw_point(Point::new(i as i32, j as i32))
                                .expect("could not draw point");
                        }
                    }
                }
                _ => {
                    for i in 0..PIXEL_SIZE {
                        for j in 0..PIXEL_SIZE {
                            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                            texture_canvas
                                .draw_point(Point::new(i as i32, j as i32))
                                .expect("could not draw point");
                        }
                    }
                }
            }
        })
        .map_err(|e| e.to_string())?;

    Ok((square_texture1, square_texture2))
}
