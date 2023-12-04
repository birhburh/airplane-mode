use macroquad::prelude::*;
use macroquad_tiled as tiled;

struct Player {
    speed: Vec2,
    position: Vec2,
    collider: Rect,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: Default::default(),
            position: Default::default(),
            collider: Default::default(),
        }
    }
}

#[macroquad::main("Airplane Mode")]
async fn main() {
    let tile_side = 32.;
    set_pc_assets_folder("assets");

    let tileset = load_texture("tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("airplane.json").await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();

    let mut player = Player {
        position: vec2(tile_side * 5., 0.),
        ..Default::default()
    };
    let mut y = 0.0;
    loop {
        clear_background(BLACK);

        set_camera(&Camera2D::from_display_rect(Rect::new(
            0.0,
            player.position.y + screen_height() / 2.0,
            tile_side * 11.,
            -tile_side * 11. * screen_height() / screen_width(),
        )));

        // draw map
        for layer in &tiled_map.raw_tiled_map.layers {
            tiled_map.draw_tiles(
                &layer.name,
                Rect::new(
                    layer.offsetx.unwrap_or_default() as f32,
                    layer.offsety.unwrap_or_default() as f32,
                    tile_side * 11.,
                    tile_side * 78.,
                ),
                None,
            );
        }

        // draw player
        {
            // sprite id from tiled
            const PLAYER_SPRITE: u32 = 15;

            let pos = player.position;
            if player.speed.x >= 0.0 {
                tiled_map.spr(
                    "airplane",
                    PLAYER_SPRITE,
                    Rect::new(pos.x, pos.y, tile_side, tile_side),
                );
            } else {
                tiled_map.spr(
                    "airplane",
                    PLAYER_SPRITE,
                    Rect::new(pos.x + tile_side, pos.y, -tile_side, tile_side),
                );
            }
        }

        // player movement control
        {
            if is_key_down(KeyCode::Right) {
                player.speed.x = 100.0;
            } else if is_key_down(KeyCode::Left) {
                player.speed.x = -100.0;
            } else if is_key_down(KeyCode::Down) {
                player.speed.y = 100.;
            } else if is_key_down(KeyCode::Up) {
                player.speed.y = -100.;
            } else {
                player.speed.x = 0.;
                player.speed.y = 0.;
            }

            player.position.x += player.speed.x * get_frame_time();
            player.position.y += player.speed.y * get_frame_time();
        }
        draw_text(&format!("FPS: {}", get_fps()), 0.0, 0.0, 16., RED);
        draw_text(&format!("HEIGHT: {}", screen_height()), 0.0, 16.0, 16., RED);
        draw_text(&format!("Y: {}", player.position.y), 0.0, 32.0, 16., RED);

        y += 1.;
        next_frame().await
    }
}
