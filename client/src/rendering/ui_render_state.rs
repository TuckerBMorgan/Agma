use storm::graphics::{
    shaders::{sprite::*}
};
use storm::Context;
use crate::AgmaClientApp;
use storm::math::{OrthographicCamera};
use storm::graphics::Buffer;
use storm::color::RGBA8;
use storm::cgmath::*;
use storm::graphics::{Texture, Uniform, std140};

pub struct UIRenderState {
    health_bar_sprite: Sprite,
    enemy_health_bar_sprite: Sprite,
    sprite_shader: SpriteShader,
    default_texture: Texture,
    health_bar_buffer: Buffer<Sprite>,
    transform_uniform: Uniform<std140::mat4>,
}

impl UIRenderState {
    pub fn new(ctx: &mut Context<AgmaClientApp>) -> UIRenderState {
        let mut transform = OrthographicCamera::new(ctx.window_logical_size());
        let transform_uniform = Uniform::new(ctx, transform.matrix());
        let sprite = Sprite {
            pos: Vector3::new(-500.0, -60.0, -1.0),
            size: Vector2::new(30, 120),
            color: RGBA8::RED,
            ..Default::default()
        };

        let enemy_sprite = Sprite {
            pos: Vector3::new(0.0, 500.0, -1.0),
            size: Vector2::new(120, 30),
            color: RGBA8::RED,
            ..Default::default()
        };

        UIRenderState {
            health_bar_sprite: sprite,
            enemy_health_bar_sprite: enemy_sprite,
            default_texture: ctx.default_texture(),
            sprite_shader: SpriteShader::new(ctx),
            health_bar_buffer: Buffer::new(ctx),
            transform_uniform
        }
    }

    pub fn configure_player_health_bar(&mut self, precent_full: f32) {
        self.health_bar_sprite.size.y = (120.0 * precent_full) as u16;
    }

    pub fn configure_enemy_health_bar(&mut self, precent_full: f32) {
        self.enemy_health_bar_sprite.size.x = (120.0 * precent_full) as u16;
    }

    pub fn render_ui(&mut self) {
        self.health_bar_buffer.set_data(&[self.health_bar_sprite, self.enemy_health_bar_sprite]);
        self.sprite_shader.draw(
            &self.transform_uniform,
            &self.default_texture,
            &[&self.health_bar_buffer],
        );
    }
}