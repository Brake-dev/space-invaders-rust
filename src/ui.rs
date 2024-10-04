use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::game::{CANVAS_HEIGHT, CANVAS_WIDTH, PIXEL_SIZE};
use crate::texture_templates::ARROW;
use crate::util::{center_x, center_y};

pub fn create_ui<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(HashMap<Rect, Texture<'a>>, HashMap<Rect, Texture<'a>>), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("./src/PressStart2P-vaV7.ttf", (16 * PIXEL_SIZE) as u16)?;

    let modal_target = Rect::new(
        center_x(CANVAS_WIDTH / 2),
        center_y(CANVAS_HEIGHT / 2),
        CANVAS_WIDTH / 2,
        CANVAS_HEIGHT / 2,
    );

    let game_over_surface = font
        .render("Game Over!")
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;

    let retry_surface = font
        .render("Retry")
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;

    let quit_surface = font
        .render("Quit")
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;

    let game_over_texture = texture_creator
        .create_texture_from_surface(&game_over_surface)
        .map_err(|e| e.to_string())?;

    let retry_texture = texture_creator
        .create_texture_from_surface(&retry_surface)
        .map_err(|e| e.to_string())?;

    let quit_texture = texture_creator
        .create_texture_from_surface(&quit_surface)
        .map_err(|e| e.to_string())?;

    let game_over_query = game_over_texture.query();
    let retry_query = retry_texture.query();
    let quit_query = quit_texture.query();

    let game_over_target = Rect::new(
        modal_target.center().x() - (game_over_query.width / 2) as i32,
        modal_target.top(),
        game_over_query.width,
        game_over_query.height,
    );

    let retry_target = Rect::new(
        modal_target.center().x() - (retry_query.width / 2) as i32,
        modal_target.top() + 200,
        retry_query.width,
        retry_query.height,
    );

    let quit_target = Rect::new(
        modal_target.center().x() - (quit_query.width / 2) as i32,
        modal_target.top() + 350,
        quit_query.width,
        quit_query.height,
    );

    let arrow_target = Rect::new(
        retry_target.left() - 200,
        retry_target.center().y() - (retry_target.height() / 2) as i32,
        ARROW[0].len() as u32 * PIXEL_SIZE * 2,
        ARROW.len() as u32 * PIXEL_SIZE * 2,
    );

    let mut modal_texture = texture_creator
        .create_texture_target(None, CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2)
        .map_err(|e| e.to_string())?;

    let mut arrow_texture = texture_creator
        .create_texture_target(None, ARROW[0].len() as u32, ARROW.len() as u32)
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut modal_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));

            texture_canvas
                .fill_rect(Rect::new(0, 0, CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2))
                .expect("could not draw rect");
        })
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut arrow_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (i, _) in ARROW.iter().enumerate() {
                for (j, val) in ARROW[i].iter().enumerate() {
                    if *val == 0 {
                        texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
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

    let mut modal_hash: HashMap<Rect, Texture> = HashMap::new();
    modal_hash.insert(modal_target, modal_texture);

    let mut ui_hash: HashMap<Rect, Texture> = HashMap::new();
    ui_hash.insert(arrow_target, arrow_texture);
    ui_hash.insert(game_over_target, game_over_texture);
    ui_hash.insert(retry_target, retry_texture);
    ui_hash.insert(quit_target, quit_texture);

    Ok((ui_hash, modal_hash))
}
