mod input;

use crate::input::InputHandler;
use macroquad::prelude::*;
use macroquad_profiler::profiler;
use macroquad_tiled as tiled;

struct Player {
    rect: Rect,
    speed: Vec2,
    frame: i32,
    time: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            rect: Default::default(),
            speed: Default::default(),
            frame: Default::default(),
            time: Default::default(),
        }
    }
}

#[macroquad::main("Airplane Mode")]
async fn main() {
    let tile_side = 32.;
    let mut input_handler = InputHandler::new();
    set_pc_assets_folder("assets");
    macroquad_profiler::profiler(Default::default());

    let tileset = load_texture("tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("airplane.json").await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();

    let mut player = Player {
        rect: Rect::new(tile_side * 5., 0., tile_side - 8., tile_side - 8.),
        ..Default::default()
    };
    // set_fullscreen(true);

    use std::time::Instant;
    let now = Instant::now();

    loop {
        // clear_background(BLACK);

        let camera_height = tile_side * 11. * screen_height() / screen_width();
        let camera_start = player.rect.y - camera_height / 2. - tile_side;
        let camera_end = player.rect.y + camera_height / 2. + tile_side;
        let camera = Camera2D {
            target: vec2(tile_side * 11. / 2., player.rect.y),
            zoom: vec2(1. / (tile_side * 11. / 2.), 1. / (camera_height / 2.)),
            ..Default::default()
        };

        // let camera = Camera2D {
        //     target: vec2(tile_side * 11. / 2., player.position.y + tile_side * 2.),
        //     zoom: vec2(1. / (tile_side * 11. / 1.5), 1. / (camera_height / 1.5)),
        //     ..Default::default()
        // };

        // let camera_width = tile_side * 78. * screen_width() / screen_height();
        // let camera =
        //     Camera2D::from_display_rect(Rect::new(0., tile_side * 78., camera_width, -tile_side * 78.));

        set_camera(&camera);

        input_handler.update();

        if input_handler.right {
            player.speed.x = 100.0;
        } else if input_handler.left {
            player.speed.x = -100.0;
        } else {
            player.speed.x = 0.;
        }
        if input_handler.down {
            player.speed.y = 100.;
        } else if input_handler.up {
            player.speed.y = -100.;
        } else {
            player.speed.y = 0.;
        }
        if is_key_down(KeyCode::Escape) {
            break;
        }

        let dx = player.speed.x * get_frame_time();
        let dy = player.speed.y * get_frame_time();
        let mut new_rect = Rect::new(
            player.rect.x + dx,
            player.rect.y + dy,
            player.rect.w,
            player.rect.h,
        );

        // draw map
        for layer in &tiled_map.raw_tiled_map.layers {
            let offsetx = layer.offsetx.unwrap_or_default() as f32;
            let offsety = layer.offsety.unwrap_or_default() as f32;
            let mut start_y = 0;
            let mut end_y = layer.height as f32 - 1.;
            let mut start_set = false;
            for (_, y, tile) in tiled_map.tiles(&layer.name, None) {
                let real_y = y as f32 * tile_side + offsety;

                if start_set && (real_y >= camera_end) {
                    end_y = (y - 1) as f32;
                    break;
                }
                if tile.is_some() {
                    if !start_set && (real_y >= camera_start) {
                        start_y = y;
                        start_set = true;
                    }
                }
            }

            let start_y_f = start_y as f32 * tile_side + offsety;
            let layer_height = (end_y - start_y as f32) + 1.;
            let src = Rect::new(0., start_y as f32, 11., layer_height);
            if start_y_f > camera_start && start_y_f < camera_end {
                tiled_map.draw_tiles(
                    &layer.name,
                    Rect::new(
                        offsetx,
                        start_y_f,
                        tile_side * 11.,
                        tile_side * layer_height,
                    ),
                    src,
                );

                // check collisions
                if new_rect.y < 0. {
                    new_rect.y = player.rect.y;
                }
                if new_rect.y > tile_side * (layer.height as f32 - 1.) {
                    new_rect.y = player.rect.y;
                }

                for (x, y, tile) in tiled_map.tiles(&layer.name, src) {
                    let tile_rect = Rect::new(
                        x as f32 * tile_side + offsetx,
                        y as f32 * tile_side + offsety,
                        tile_side,
                        tile_side,
                    );
                    if y as f32 > start_y as f32 + camera_height / tile_side + 1. {
                        break;
                    }
                    if tile_rect.y < (player.rect.y + camera_height / 2. + tile_side) {
                        if let Some(tile) = tile {
                            if tile.id != 0 {
                                draw_rectangle_lines(
                                    x as f32 * tile_side + offsetx,
                                    tile_rect.y,
                                    tile_side,
                                    tile_side,
                                    2.,
                                    YELLOW,
                                );
                            }
                        }
                    }
                    if tile_rect.y < new_rect.y + tile_side
                        && tile_rect.y > new_rect.y - tile_side
                        && tile_rect.x < new_rect.x + tile_side
                        && tile_rect.x > new_rect.x - tile_side
                    {
                        if let Some(tile) = tile {
                            if tile.id != 0 {
                                let color = if new_rect.overlaps(&tile_rect) {
                                    LIME
                                } else {
                                    PINK
                                };
                                draw_rectangle_lines(
                                    tile_rect.x,
                                    tile_rect.y,
                                    tile_rect.w,
                                    tile_rect.h,
                                    3.,
                                    color,
                                );
                                if new_rect.overlaps(&tile_rect) {
                                    if tile_rect.right() < player.rect.left() ||
                                    tile_rect.left() > player.rect.right()
                                    {
                                        new_rect.x = player.rect.x;
                                    }
                                    if tile_rect.bottom() < player.rect.top() ||
                                       tile_rect.top() > player.rect.bottom()
                                    {
                                        new_rect.y = player.rect.y;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let x_sign = if new_rect.x < player.rect.x {
            -1.
        } else if new_rect.x > player.rect.x {
            1.
        } else {
            0.
        };
        let y_sign = if new_rect.y < player.rect.y {
            -1.
        } else if new_rect.y > player.rect.y {
            1.
        } else {
            0.
        };
        draw_line(
            player.rect.x + player.rect.w / 2.,
            player.rect.y + player.rect.h / 2.,
            player.rect.x + player.rect.w / 2. + x_sign * 100.,
            player.rect.y + player.rect.h / 2. + y_sign % 2. * 100.,
            3.,
            PURPLE,
        );

        {
            player.rect = new_rect;
        }

        // draw player
        {
            // sprite id from tiled
            const STAND_SPRITE: u32 = 15;
            const RUN_1_SPRITE: u32 = 19;
            const RUN_2_SPRITE: u32 = 23;

            let running = player.speed.x != 0. || player.speed.y != 0.;
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

            let pos = player.rect;
            let xdiff = tile_side - player.rect.w;
            let ydiff = tile_side - player.rect.h;
            let rect = if player.speed.x >= 0.0 {
                Rect::new(pos.x - xdiff / 2., pos.y - ydiff, tile_side, tile_side)
            } else {
                Rect::new(pos.x + tile_side - xdiff / 2., pos.y - ydiff, -tile_side, tile_side)
            };
            tiled_map.spr("airplane", sprite, rect);
        }
        draw_rectangle_lines(
            player.rect.x,
            player.rect.y,
            player.rect.w,
            player.rect.h,
            2.,
            BLUE,
        );
        draw_rectangle_lines(
            0.,
            player.rect.y - camera_height / 2.,
            tile_side * 11.,
            camera_height,
            2.,
            BLUE,
        );
        set_default_camera();

        #[cfg(target_os = "android")]
        input_handler.draw();
        // draw_text(&format!("{:?}", ends), 0.0, 64.0, 16., RED);
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
