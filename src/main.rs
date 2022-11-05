use macroquad::prelude::*;

struct Turret {
    rot: f32,           // 車体を基準とした砲塔の角度
    angular_speed: f32, // 砲塔の旋回速度
    texture: Texture2D,
    image_scale: f32, // 画像描画時の拡大率
}

struct Tank {
    pos: Vec2,          // 車体中心位置
    rot: f32,           // 前方向を表す角度[deg] -y方向が0、時計回りが正とする
    speed: i32,         // 1フレームあたりの移動距離
    angular_speed: f32, // 1フレームあたりの旋回速度
    texture: Texture2D,
    image_scale: f32, // 画像描画時の拡大率
    turret: Turret,   // 砲塔
}

#[macroquad::main("woo-tank-macroquad")]
async fn main() {
    let mut player = Tank {
        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
        rot: 20.,
        speed: 5,
        angular_speed: 5.,
        texture: load_texture("image/tank_blue.png").await.unwrap(),
        image_scale: 1.8,
        turret: Turret {
            rot: 0.,
            angular_speed: 5.,
            image_scale: 0.6,
            texture: load_texture("image/turret_blue.png").await.unwrap(),
        },
    };

    loop {
        // 右旋回
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            player.rot += player.angular_speed;
        }

        // 左旋回
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            player.rot -= player.angular_speed;
        }

        let rot_rad = player.rot.to_radians();
        // 速度(移動量)
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
        // 車体描画
        draw_texture_ex(
            player.texture,
            player.pos.x - player.texture.width() / 2. * player.image_scale,
            player.pos.y - player.texture.height() / 2. * player.image_scale,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    player.texture.width() * player.image_scale,
                    player.texture.height() * player.image_scale,
                )),
                rotation: rot_rad,
                ..Default::default()
            },
        );
        // 砲塔描画
        draw_texture_ex(
            player.turret.texture,
            player.pos.x - player.turret.texture.width() / 2. * player.turret.image_scale,
            player.pos.y - player.turret.texture.height() / 2. * player.turret.image_scale,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    player.turret.texture.width() * player.turret.image_scale,
                    player.turret.texture.height() * player.turret.image_scale,
                )),
                rotation: rot_rad,
                ..Default::default()
            },
        );

        next_frame().await
    }
}
