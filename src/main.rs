use macroquad::prelude::*;

struct Tank {
    pos: Vec2,  // 位置
    rot: f32,   // 前方向を表す角度[deg] -y方向が0、時計回りが正とする
    speed: i32, // 1フレームあたりの移動距離
    texture: Texture2D,
}

#[macroquad::main("woo-tank-macroquad")]
async fn main() {
    let mut player = Tank {
        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
        rot: 20.,
        speed: 5,
        texture: load_texture("image/tank_blue.png").await.unwrap(),
    };

    loop {
        // 右旋回
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            player.rot += 5.;
        }

        // 左旋回
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            player.rot -= 5.;
        }

        let rot_rad = player.rot.to_radians();
        let mut vel = Vec2::new(0., 0.);

        // 前進
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            // y軸に反転
            vel += Vec2::new(rot_rad.sin(), -rot_rad.cos()) * player.speed as f32;
        }
        // 後進
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            // y軸に反転
            vel -= Vec2::new(rot_rad.sin(), -rot_rad.cos()) * player.speed as f32;
        }
        player.pos += vel;

        clear_background(LIGHTGRAY);

        draw_texture_ex(
            player.texture,
            player.pos.x,
            player.pos.y,
            WHITE,
            DrawTextureParams {
                rotation: rot_rad,
                ..Default::default()
            },
        );

        next_frame().await
    }
}
