use macroquad::prelude::*;

// 砲塔
struct Turret {
    angle: f32,         // 車体を基準とした砲塔の角度[deg]
    angular_speed: f32, // 砲塔の旋回速度[deg]
    aim_mouse: bool,    // 真なら砲塔旋回をマウスカーソル追従、偽ならキー操作で行う
    texture: Texture2D,
    width: f32,  // 幅
    height: f32, // 長さ
}
// 車体
struct Body {
    pos: Vec2,          // 車体中心位置
    angle: f32,         // 前方向を表す角度[deg]
    speed: i32,         // 1フレームあたりの移動距離(速さ)
    angular_speed: f32, // 1フレームあたりの旋回速度
    texture: Texture2D,
    width: f32,     // 幅
    height: f32,    // 長さ
    turret: Turret, // 砲塔
}

impl Body {
    fn new(body_texture: Texture2D, turret_texture: Texture2D) -> Self {
        // 元画像のサイズに拡大率を掛けて描画したいサイズを求める
        let body_width = body_texture.width() * 1.8;
        let body_height = body_texture.height() * 1.8;

        let turret_width = turret_texture.width() * 0.6;
        let turret_height = turret_texture.height() * 0.6;

        Body {
            pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
            angle: 20.,
            speed: 5,
            angular_speed: 5.,
            texture: body_texture,
            width: body_width,
            height: body_height,
            turret: Turret {
                angle: 0.,
                angular_speed: 5.,
                aim_mouse: true,
                texture: turret_texture,
                width: turret_width,
                height: turret_height,
            },
        }
    }
}

#[macroquad::main("woo-tank-macroquad")]
async fn main() {
    let mut player = Body::new(
        load_texture("image/tank_blue.png").await.unwrap(),
        load_texture("image/turret_blue.png").await.unwrap(),
    );

    loop {
        // 車体右旋回
        if is_key_down(KeyCode::D) {
            player.angle += player.angular_speed;
        }

        // 車体左旋回
        if is_key_down(KeyCode::A) {
            player.angle -= player.angular_speed;
        }
        // 1周したら戻す
        player.angle %= 360.;

        if is_key_pressed(KeyCode::C) {
            // マウス追従/キー操作モード切り替え
            player.turret.aim_mouse = !player.turret.aim_mouse;
        }

        let mouse_pos: Vec2 = mouse_position().into();
        if player.turret.aim_mouse {
            let turret_vec = angle2vec2((player.turret.angle + player.angle).to_radians());
            // プレイヤーからマウスカーソルへ向かうベクトル
            let player2mouse_vec =
                Vec2::new(mouse_pos.x - player.pos.x, mouse_pos.y - player.pos.y);

            // 砲の方向とマウスカーソルを指す方向とのなす角
            let angle_diff_deg = turret_vec.angle_between(player2mouse_vec).to_degrees();
            if angle_diff_deg.abs() > 10. {
                // 角度差が一定以上であれば、定速で旋回
                player.turret.angle += player.turret.angular_speed * angle_diff_deg.signum()
            } else {
                // 角度差が一定以下の場合、ease-out補間で旋回速度を求める
                // -1 <= t <= 1
                let t = angle_diff_deg / 10.;
                player.turret.angle += t * (2. - t);
            }
        } else {
            // 砲塔右旋回
            if is_key_down(KeyCode::Right) {
                player.turret.angle += player.turret.angular_speed;
            }
            // 砲塔左旋回
            if is_key_down(KeyCode::Left) {
                player.turret.angle -= player.turret.angular_speed;
            }
        }
        // 1周したら戻す
        player.turret.angle %= 360.;

        let body_angle_rad = player.angle.to_radians();
        // 速度(移動量)
        let mut vel = Vec2::new(0., 0.);

        // 前進
        if is_key_down(KeyCode::W) {
            // y軸に反転
            vel += angle2vec2(body_angle_rad) * player.speed as f32;
        }
        // 後進
        if is_key_down(KeyCode::S) {
            // y軸に反転
            vel -= angle2vec2(body_angle_rad) * player.speed as f32;
        }

        let moved_pos = player.pos + vel;
        // 左右端より外側に出ていなければ動かす
        if 0. < moved_pos.x - player.width / 2. && moved_pos.x + player.width / 2. < screen_width()
        {
            player.pos.x = moved_pos.x;
        }
        // 上下端より外側に出ていなければ
        if 0. < moved_pos.y - player.height / 2.
            && moved_pos.y + player.height / 2. < screen_height()
        {
            player.pos.y = moved_pos.y;
        }

        // 背景色描画
        clear_background(LIGHTGRAY);
        // 車体描画
        draw_texture_ex(
            player.texture,
            player.pos.x - player.width / 2.,
            player.pos.y - player.height / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(player.width, player.height)),
                rotation: body_angle_rad,
                ..Default::default()
            },
        );
        // 砲塔描画
        draw_texture_ex(
            player.turret.texture,
            player.pos.x - player.turret.width / 2.,
            player.pos.y - player.turret.height / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(player.turret.width, player.turret.height)),
                rotation: body_angle_rad + player.turret.angle.to_radians(),
                ..Default::default()
            },
        );
        // マウス追従モードならばカーソル位置に丸を表示
        if player.turret.aim_mouse {
            draw_circle_lines(mouse_pos.x, mouse_pos.y, 10., 3., BLACK);
        }

        next_frame().await
    }
}

// 角度は-y軸方向が0、時計回りが正
fn angle2vec2(angle_rad: f32) -> Vec2 {
    Vec2::new(angle_rad.sin(), -angle_rad.cos())
}
