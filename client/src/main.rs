use shared_code::*;
use core::cmp::min;
use storm::color::RGBA8;
use storm::graphics::{ClearMode, DisplayMode, Vsync,Texture,TextureFiltering, WindowSettings, DepthTest};

use std::sync::mpsc::{Receiver};
use storm::{event::*, Context, App, start, event, cgmath::*};
use storm::graphics::{Buffer};

use std::time::Duration;

use log::LevelFilter;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
mod game;
use game::*;

mod networking;
mod rendering;

pub use networking::*;

static TEXTURE_A: &[u8] = include_bytes!("resources/images/floor.png");

pub enum AppState {
    WorkingOnConnection,
    Game
}

pub struct Preamble {
    server_connection_info: Receiver<ServerConnectionInfo>
}

pub struct AgmaClientApp {
    app_state: AppState,
    rift: Option<Rift>,
    preamble: Preamble
}

impl App for AgmaClientApp {
    fn new(ctx: &mut Context<Self>) -> Self {
        ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
        
        let handshake = preform_handshake(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(24, 19, 122, 147)), 34258));
        let preamble = Preamble{server_connection_info: handshake};
        AgmaClientApp {
            app_state: AppState::WorkingOnConnection,
            rift: None,//Rift::new(ctx)
            preamble
        }
    }

    fn on_close_requested(&mut self, ctx: &mut Context<Self>) {
        ctx.request_stop();
    }

    fn on_update(&mut self, ctx: &mut Context<Self>, delta: f32) {
        match self.app_state {
            AppState::WorkingOnConnection => {
                let maybe = self.preamble.server_connection_info.try_iter();
                for thing in maybe {
                    self.rift = Some(Rift::new(ctx, thing.client_id, thing.port));
                    self.app_state = AppState::Game;
                    break;
                }
            },
            AppState::Game => {
                self.rift.as_mut().unwrap().update(ctx, delta);
            }
        }


    }

    fn on_cursor_pressed(
        &mut self,
        ctx: &mut Context<Self>,
        button: event::CursorButton,
        physical_pos: storm::cgmath::Vector2<f32>,
        normalized_pos: storm::cgmath::Vector2<f32>,
    ) {
        match self.app_state {
            AppState::Game => {
                self.rift.as_mut().unwrap().on_cursor_pressed(ctx, button, physical_pos, normalized_pos);
            }
            _ => {

            }
        }

    }

    fn on_cursor_released(
        &mut self,
        ctx: &mut Context<Self>,
        button: event::CursorButton,
        physical_pos: storm::cgmath::Vector2<f32>,
        normalized_pos: storm::cgmath::Vector2<f32>,
    ) {
        match self.app_state {

            AppState::Game => {
                self.rift.as_mut().unwrap().on_cursor_released(ctx, button, physical_pos, normalized_pos);
            }
            _ => {
                
            }
        }

    }

    fn on_cursor_delta(&mut self, ctx: &mut Context<AgmaClientApp>, delta: storm::cgmath::Vector2<f32>, focused: bool) {
        match self.app_state {
            AppState::Game => {
                self.rift.as_mut().unwrap().on_cursor_delta(ctx, delta, focused);
            }
            _ => {
                
            }
        }

    }

    fn on_key_pressed(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton, is_repeat: bool) {
        if is_repeat {
            return;
        }
        match self.app_state {
            AppState::Game => {
                self.rift.as_mut().unwrap().on_key_pressed(ctx, key, is_repeat);
            }
            _ => {
                
            }
        }

    }

    fn on_key_released(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton) {
        match self.app_state {
            AppState::Game => {
                self.rift.as_mut().unwrap().on_key_released(ctx, key);
            }
            _ => {
                
            }
        }

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