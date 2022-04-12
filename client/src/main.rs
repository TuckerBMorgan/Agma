use shared_code::*;
use core::cmp::min;
use storm::color::RGBA8;
use storm::graphics::{ClearMode, DisplayMode, Vsync,Texture,TextureFiltering, WindowSettings, DepthTest};
use storm::cgmath::*;

use storm::{color::*, event::*, graphics::*, math::*, *};
use storm::graphics::{Buffer};

use std::time::Duration;

use log::{info};
use log::LevelFilter;

mod game;
use game::*;

mod networking;
mod rendering;

static TEXTURE_A: &[u8] = include_bytes!("resources/images/floor.png");


pub struct AgmaClientApp {
    rift: Rift
}

impl App for AgmaClientApp {
    fn new(ctx: &mut Context<Self>) -> Self {
        ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
        AgmaClientApp {
            rift: Rift::new(ctx)
        }
    }

    fn on_close_requested(&mut self, ctx: &mut Context<Self>) {
        ctx.request_stop();
    }

    fn on_update(&mut self, ctx: &mut Context<Self>, delta: f32) {
        self.rift.update(ctx, delta);    
    }

    fn on_cursor_pressed(
        &mut self,
        ctx: &mut Context<Self>,
        button: event::CursorButton,
        physical_pos: cgmath::Vector2<f32>,
        normalized_pos: cgmath::Vector2<f32>,
    ) {

        self.rift.on_cursor_pressed(ctx, button, physical_pos, normalized_pos);
    }

    fn on_cursor_released(
        &mut self,
        ctx: &mut Context<Self>,
        button: event::CursorButton,
        physical_pos: cgmath::Vector2<f32>,
        normalized_pos: cgmath::Vector2<f32>,
    ) {
        self.rift.on_cursor_released(ctx, button, physical_pos, normalized_pos);
    }

    fn on_cursor_delta(&mut self, ctx: &mut Context<AgmaClientApp>, delta: cgmath::Vector2<f32>, focused: bool) {
        self.rift.on_cursor_delta(ctx, delta, focused);
    }


    fn on_key_pressed(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton, is_repeat: bool) {
        if is_repeat {
            return;
        }
        self.rift.on_key_pressed(ctx, key, is_repeat);
    }

    fn on_key_released(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton) {
        self.rift.on_key_released(ctx, key);
    }
}

fn main() {
    let _ = simple_logging::log_to_file("client.log", LevelFilter::Info);
    start::<AgmaClientApp>(
        WindowSettings {
            title: String::from("Agma"),
            display_mode: DisplayMode::Windowed {
                width: 1280,
                height: 1024,
                resizable: true,
            },
        vsync: Vsync::Disabled,
        }
    );
}