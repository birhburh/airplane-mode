use macroquad::prelude::*;
use macroquad_profiler::profiler;
use macroquad_tiled as tiled;

struct Player {
    speed: Vec2,
    position: Vec2,
    frame: i32,
    time: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: Default::default(),
            position: Default::default(),
            frame: Default::default(),
            time: Default::default(),
        }
    }
}

#[macroquad::main("Airplane Mode")]
async fn main() {
    let tile_side = 32.;
    set_pc_assets_folder("assets");
    macroquad_profiler::profiler(Default::default());

    let tileset = load_texture("tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("airplane.json").await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();

    let mut player = Player {
        position: vec2(tile_side * 5., 0.),
        ..Default::default()
    };
    // set_fullscreen(true);

    use std::time::Instant;
    let now = Instant::now();

    loop {
        // clear_background(BLACK);

        let camera_height = tile_side * 11. * screen_height() / screen_width();
        let camera = Camera2D {
            target: vec2(tile_side * 11. / 2., player.position.y),
            zoom: vec2(1. / (tile_side * 11. / 2.), 1. / (camera_height / 2.)),
            ..Default::default()
        };
        set_camera(&camera);

        // draw map
        for layer in &tiled_map.raw_tiled_map.layers {
            let offsetx = layer.offsetx.unwrap_or_default() as f32;
            let offsety = layer.offsety.unwrap_or_default() as f32;
            let mut new_y = 0;
            for (_, y, tile) in tiled_map.tiles(&layer.name, None) {
                if tile.is_some() {
                    let real_y = y as f32 * tile_side + offsety;

                    if real_y > (player.position.y - camera_height / 2. - tile_side) {
                        new_y = y;
                        break;
                    }
                }
            }

            let new_y_f = new_y as f32 * tile_side + offsety;
            let src = Rect::new(0., new_y as f32, 11., camera_height / tile_side + 2.);
            let new_y = new_y_f;
            if new_y < (player.position.y + camera_height / 2. + tile_side) {
                tiled_map.draw_tiles(
                    &layer.name,
                    Rect::new(
                        offsetx,
                        new_y,
                        tile_side * 11.,
                        camera_height + tile_side * 2.,
                    ),
                    src,
                );
            }
            for (x, y, tile) in tiled_map.tiles(&layer.name, None) {
                if y > new_y as u32 + 10 {
                    break;
                }
                if let Some(tile) = tile {
                    if tile.id != 0 {
                        draw_rectangle_lines(
                            x as f32 * tile_side + offsetx,
                            y as f32 * tile_side + offsety,
                            tile_side,
                            tile_side,
                            1.,
                            YELLOW,
                        );
                    }
                }
            }
        }

        let mut running = true;
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
            } else if is_key_down(KeyCode::Escape) {
                break;
            } else {
                player.speed.x = 0.;
                player.speed.y = 0.;
                running = false;
            }

            // check collisions
            if player.position.x + player.speed.x * get_frame_time() > tile_side * 9.
                || player.position.x + player.speed.x * get_frame_time() < tile_side
                || player.position.y + player.speed.y * get_frame_time() < 0.
                || player.position.y + player.speed.y * get_frame_time() > tile_side * 77.
            {
                player.speed.x = 0.;
                player.speed.y = 0.;
            }

            player.position.x += player.speed.x * get_frame_time();
            player.position.y += player.speed.y * get_frame_time();
        }

        // draw player
        {
            // sprite id from tiled
            const STAND_SPRITE: u32 = 15;
            const RUN_1_SPRITE: u32 = 19;
            const RUN_2_SPRITE: u32 = 23;

            let sprite = if running {
                player.time += get_frame_time();
                if player.time > 1. / 12 as f32 {
                    player.frame += 1;
                    player.time = 0.0;
                }
                player.frame %= 2;

                if player.frame == 0 {
                    RUN_1_SPRITE
                } else {
                    RUN_2_SPRITE
                }
            } else {
                STAND_SPRITE
            };

            let pos = player.position;
            let rect = if player.speed.x >= 0.0 {
                Rect::new(pos.x, pos.y, tile_side, tile_side)
            } else {
                Rect::new(pos.x + tile_side, pos.y, -tile_side, tile_side)
            };
            tiled_map.spr("airplane", sprite, rect);
        }
        draw_rectangle_lines(
            player.position.x,
            player.position.y,
            tile_side,
            tile_side,
            2.,
            BLUE,
        );
        set_default_camera();
        // draw_text(&format!("{:?}", camera.target), 0.0, 64.0, 16., RED);
        draw_circle(
            (screen_width() - 2.) / 2.,
            (screen_height() - 2.) / 2.,
            2.,
            BLUE,
        );

        let elapsed = now.elapsed();
        let font_size = 30.;
        let text = format!("TIME: {:.2?}", elapsed);
        let text_size = measure_text(&text, None, font_size as _, 1.0);
        draw_text(
            &text,
            screen_width() / 2. - text_size.width / 2.,
            text_size.height + 1.,
            font_size,
            DARKGRAY,
        );

        profiler(Default::default());

        next_frame().await;
    }
}
