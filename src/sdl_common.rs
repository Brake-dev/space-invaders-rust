use sdl2::{render, video, EventPump, EventSubsystem};

use crate::game::{CANVAS_HEIGHT, CANVAS_WIDTH};

pub struct RetryEvent;
pub struct ContinueEvent;

pub fn init() -> Result<(render::Canvas<video::Window>, EventSubsystem, EventPump), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Space Invaders: Rust",
            CANVAS_WIDTH as u32,
            CANVAS_HEIGHT as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let event = sdl_context.event()?;
    let event_pump = sdl_context.event_pump()?;

    event.register_custom_event::<RetryEvent>()?;
    event.register_custom_event::<ContinueEvent>()?;

    Ok((canvas, event, event_pump))
}
