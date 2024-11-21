use std::collections::{HashMap, HashSet};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventSubsystem;

use crate::game::{State, CANVAS_HEIGHT, CANVAS_WIDTH, PIXEL_SIZE};
use crate::sdl_common::{ContinueEvent, RetryEvent};
use crate::texture_templates::ARROW;
use crate::util::{center_x, center_y};

#[derive(Clone)]
pub struct UI {
    pub targets: [Rect; 2],
    pub cursor_pos: usize,
    prev_keys: HashSet<Keycode>,
}

impl UI {
    pub fn new(retry_target: Rect, quit_target: Rect) -> UI {
        UI {
            targets: [retry_target, quit_target],
            cursor_pos: 0,
            prev_keys: HashSet::new(),
        }
    }

    pub fn update_cursor(&mut self) {
        if self.cursor_pos == 0 {
            self.cursor_pos = 1;
        } else {
            self.cursor_pos = 0;
        }
    }

    pub fn get_cursor_target(&self) -> Rect {
        let target = self.targets[self.cursor_pos];

        Rect::new(
            target.left() - 300,
            target.center().y() - (target.height() / 2) as i32,
            ARROW[0].len() as u32 * PIXEL_SIZE as u32 * 2,
            ARROW.len() as u32 * PIXEL_SIZE as u32 * 2,
        )
    }

    pub fn select(&self, event: &EventSubsystem, game_state: &State) {
        if self.cursor_pos == 0 {
            if game_state == &State::GameOver || game_state == &State::Win {
                let result = event.push_custom_event(RetryEvent);
                match result {
                    Ok(_) => (),
                    Err(_) => panic!("Error handling retry event"),
                }
            } else if game_state == &State::Paused {
                let result = event.push_custom_event(ContinueEvent);
                match result {
                    Ok(_) => (),
                    Err(_) => panic!("Error handling retry event"),
                }
            }
        } else {
            let result = event.push_event(Event::Quit { timestamp: (0) });
            match result {
                Ok(_) => (),
                Err(_) => panic!("Error handling quit event"),
            }
        }
    }

    pub fn update(&mut self, keys: &HashSet<Keycode>, event: &EventSubsystem, game_state: &State) {
        let new_keys = keys - &self.prev_keys;
        let old_keys = &self.prev_keys - keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            if game_state != &State::Playing {
                if new_keys.contains(&Keycode::Up) || new_keys.contains(&Keycode::Down) {
                    self.update_cursor();
                }

                if new_keys.contains(&Keycode::Return) || new_keys.contains(&Keycode::Space) {
                    self.select(&event, &game_state);
                }
            }
        }

        self.prev_keys = keys.clone();
    }

    pub fn get_ui_targets_base_on_state<'a>(
        &self,
        ui_targets_hash: &HashMap<String, Rect>,
        game_state: &State,
    ) -> HashMap<String, Rect> {
        let mut keys = [String::from(""), String::from(""), String::from("")];

        if game_state == &State::GameOver {
            keys = [
                String::from("game over"),
                String::from("retry"),
                String::from("quit"),
            ];
        } else if game_state == &State::Paused {
            keys = [
                String::from(""),
                String::from("continue"),
                String::from("quit"),
            ];
        } else if game_state == &State::Win {
            keys = [
                String::from("victory"),
                String::from("retry"),
                String::from("quit"),
            ];
        }

        let mut ui_targets = ui_targets_hash.clone();
        ui_targets.retain(|k, _| keys.contains(k));

        return ui_targets;
    }
}

pub fn create_ui<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<
    (
        HashMap<Rect, Texture<'a>>,
        Texture<'a>,
        HashMap<String, Texture<'a>>,
        HashMap<String, Rect>,
        Rect,
    ),
    String,
> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("./src/PressStart2P-vaV7.ttf", (16 * PIXEL_SIZE) as u16)?;

    let modal_target = Rect::new(
        center_x(CANVAS_WIDTH / 2),
        center_y(CANVAS_HEIGHT / 2),
        (CANVAS_WIDTH / 2) as u32,
        (CANVAS_HEIGHT / 2) as u32,
    );

    let game_over_surface = font
        .render("Game Over!")
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;

    let victory_surface = font
        .render("Victory!")
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;

    let continue_surface = font
        .render("Continue")
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

    let victory_texture = texture_creator
        .create_texture_from_surface(&victory_surface)
        .map_err(|e| e.to_string())?;

    let continue_texture = texture_creator
        .create_texture_from_surface(&continue_surface)
        .map_err(|e| e.to_string())?;

    let retry_texture = texture_creator
        .create_texture_from_surface(&retry_surface)
        .map_err(|e| e.to_string())?;

    let quit_texture = texture_creator
        .create_texture_from_surface(&quit_surface)
        .map_err(|e| e.to_string())?;

    let game_over_query = game_over_texture.query();
    let victory_query = victory_texture.query();
    let continue_query = continue_texture.query();
    let retry_query = retry_texture.query();
    let quit_query = quit_texture.query();

    let game_over_target = Rect::new(
        modal_target.center().x() - (game_over_query.width / 2) as i32,
        modal_target.top(),
        game_over_query.width,
        game_over_query.height,
    );

    let victory_target = Rect::new(
        modal_target.center().x() - (victory_query.width / 2) as i32,
        modal_target.top(),
        victory_query.width,
        victory_query.height,
    );

    let continue_target = Rect::new(
        modal_target.center().x() - (continue_query.width / 2) as i32,
        modal_target.top() + 200,
        continue_query.width,
        continue_query.height,
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

    let default_target = Rect::new(0, 0, 0, 0);

    let mut modal_texture = texture_creator
        .create_texture_target(None, (CANVAS_WIDTH / 2) as u32, (CANVAS_HEIGHT / 2) as u32)
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
                .fill_rect(Rect::new(
                    0,
                    0,
                    (CANVAS_WIDTH / 2) as u32,
                    (CANVAS_HEIGHT / 2) as u32,
                ))
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

    let mut modal_hash: HashMap<Rect, Texture> = HashMap::new();
    modal_hash.insert(modal_target, modal_texture);

    let mut ui_texture_hash: HashMap<String, Texture> = HashMap::new();
    ui_texture_hash.insert(String::from("game_over"), game_over_texture);
    ui_texture_hash.insert(String::from("victory"), victory_texture);
    ui_texture_hash.insert(String::from("continue"), continue_texture);
    ui_texture_hash.insert(String::from("retry"), retry_texture);
    ui_texture_hash.insert(String::from("quit"), quit_texture);

    let mut ui_target_hash: HashMap<String, Rect> = HashMap::new();
    ui_target_hash.insert(String::from("game_over"), game_over_target);
    ui_target_hash.insert(String::from("victory"), victory_target);
    ui_target_hash.insert(String::from("continue"), continue_target);
    ui_target_hash.insert(String::from("retry"), retry_target);
    ui_target_hash.insert(String::from("quit"), quit_target);

    Ok((
        modal_hash,
        arrow_texture,
        ui_texture_hash,
        ui_target_hash,
        default_target,
    ))
}
