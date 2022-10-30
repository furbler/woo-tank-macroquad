use macroquad::prelude::*;
use std::f32::consts::PI;

#[macroquad::main("woo-tank-macroquad")]
async fn main() {
    let tank_blue = load_texture("image/tank_blue.png").await.unwrap();

    loop {
        clear_background(LIGHTGRAY);

        draw_texture_ex(
            tank_blue,
            32. as f32,
            32. as f32,
            WHITE,
            DrawTextureParams {
                rotation: PI / 6.,
                ..Default::default()
            },
        );

        next_frame().await
    }
}
