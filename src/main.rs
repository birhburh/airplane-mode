mod input;

use crate::input::InputHandler;
use macroquad::prelude::*;
use macroquad_profiler::profiler;
use macroquad_tiled::{self as tiled, Map};

const TILE_SIDE: f32 = 32.;

#[derive(Default)]
struct Player {
    rect: Rect,
    speed: Vec2,
    frame: i32,
    time: f32,
    camera_rect: Rect,
}

#[derive(Default)]
struct Layer {
    name: String,
    height: u32,
    offsetx: f32,
    offsety: f32,
    start_y: f32,
    end_y: f32,
    start_set: bool,
}

impl Player {
    fn new() -> Self {
        let camera_width = TILE_SIDE * 11.;
        let camera_height = camera_width * screen_height() / screen_width();
        Player {
            rect: Rect::new(TILE_SIDE * 5., 0., TILE_SIDE - 8., TILE_SIDE - 8.),
            camera_rect: Rect::new(0., 0., camera_width, camera_height),
            ..Default::default()
        }
    }

    fn draw(&mut self, tiled_map: &Map, debug_mode: bool) {
        // sprite id from tiled
        const STAND_SPRITE: u32 = 15;
        const RUN_1_SPRITE: u32 = 19;
        const RUN_2_SPRITE: u32 = 23;

        let running = self.speed.x != 0. || self.speed.y != 0.;
        let sprite = if running {
            self.time += get_frame_time();
            if self.time > 1. / 12 as f32 {
                self.frame += 1;
                self.time = 0.0;
            }
            self.frame %= 2;

            if self.frame == 0 {
                RUN_1_SPRITE
            } else {
                RUN_2_SPRITE
            }
        } else {
            STAND_SPRITE
        };

        let pos = self.rect;
        let xdiff = TILE_SIDE - self.rect.w;
        let ydiff = TILE_SIDE - self.rect.h;
        let rect = if self.speed.x >= 0.0 {
            Rect::new(pos.x - xdiff / 2., pos.y - ydiff, TILE_SIDE, TILE_SIDE)
        } else {
            Rect::new(
                pos.x + TILE_SIDE - xdiff / 2.,
                pos.y - ydiff,
                -TILE_SIDE,
                TILE_SIDE,
            )
        };
        tiled_map.spr("airplane", sprite, rect);
        if debug_mode {
            draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 2., BLUE);
        }
    }

    fn camera(&self) -> Camera2D {
        Camera2D {
            target: vec2(self.camera_rect.w / 2., self.rect.y),
            zoom: vec2(
                1. / (self.camera_rect.w / 2.),
                1. / (self.camera_rect.h / 2.),
            ),
            ..Default::default()
        }

        // Camera2D {
        //     target: vec2(TILE_SIDE * 11. / 2., player.position.y + TILE_SIDE * 2.),
        //     zoom: vec2(1. / (TILE_SIDE * 11. / 1.5), 1. / (camera_height / 1.5)),
        //     ..Default::default()
        // }

        // let camera_width = TILE_SIDE * 78. * screen_width() / screen_height();
        //     Camera2D::from_display_rect(Rect::new(0., TILE_SIDE * 78., camera_width, -TILE_SIDE * 78.))
    }
}

impl Layer {
    fn update(&mut self, tiled_map: &Map, player: &mut Player) {
        let offsety = self.offsety;
        self.start_y = 0.;
        self.end_y = self.height as f32 - 1.;
        self.start_set = false;
        for (_, y, tile) in tiled_map.tiles(&self.name, None) {
            let real_y = y as f32 * TILE_SIDE + offsety;

            let camera_end = player.camera_rect.y + player.camera_rect.h + TILE_SIDE;
            if self.start_set && (real_y >= camera_end) {
                self.end_y = (y - 1) as f32;
                break;
            }
            if tile.is_some() {
                if !self.start_set && (real_y >= player.camera_rect.y) {
                    self.start_y = y as f32;
                    self.start_set = true;
                }
            }
        }
    }

    fn draw(&self, tiled_map: &Map, player: &mut Player, debug_mode: bool) {
        if self.start_set {
            let layer_height = (self.end_y - self.start_y as f32) + 1.;
            let src = Rect::new(0., self.start_y as f32, 11., layer_height);

            let offsetx = self.offsetx;
            let offsety = self.offsety;
            tiled_map.draw_tiles(
                &self.name,
                Rect::new(
                    offsetx,
                    src.y as f32 * TILE_SIDE + offsety,
                    TILE_SIDE * src.w,
                    TILE_SIDE * src.h,
                ),
                src,
            );

            let collision_rect = Rect::new(
                player.rect.x - (player.rect.w * 2.) / 2.,
                player.rect.y - (player.rect.h * 2.) / 2.,
                player.rect.w * 3.,
                player.rect.h * 3.,
            );

            if debug_mode {
                draw_rectangle_lines(
                    collision_rect.left(),
                    collision_rect.top(),
                    collision_rect.w,
                    collision_rect.h,
                    2.,
                    BLUE,
                );
            }
            let dx = player.speed.x * get_frame_time();
            let dy = player.speed.y * get_frame_time();

            for (x, y, tile) in tiled_map.tiles(&self.name, src) {
                let tile_rect = Rect::new(
                    x as f32 * TILE_SIDE + offsetx,
                    y as f32 * TILE_SIDE + offsety,
                    TILE_SIDE,
                    TILE_SIDE,
                );

                if debug_mode {
                    if tile_rect.y < (player.rect.y + player.camera_rect.h / 2. + TILE_SIDE) {
                        if let Some(tile) = tile {
                            if tile.id != 0 {
                                draw_rectangle_lines(
                                    x as f32 * TILE_SIDE + offsetx,
                                    tile_rect.y,
                                    TILE_SIDE,
                                    TILE_SIDE,
                                    2.,
                                    YELLOW,
                                );
                            }
                        }
                    }
                }
                if tile_rect.overlaps(&collision_rect) {
                    if let Some(tile) = tile {
                        if tile.id != 0 {
                            let mut new_rect = Rect::new(
                                player.rect.x + dx,
                                player.rect.y,
                                player.rect.w,
                                player.rect.h,
                            );

                            let overlaps_x = new_rect.overlaps(&tile_rect);
                            if overlaps_x {
                                player.speed.x = 0.;
                            }

                            new_rect.x = player.rect.x;
                            new_rect.y += dy;
                            let overlaps_y = new_rect.overlaps(&tile_rect);
                            if overlaps_y {
                                player.speed.y = 0.;
                            }

                            if debug_mode {
                                let color = if overlaps_x || overlaps_y { LIME } else { PINK };
                                draw_rectangle_lines(
                                    tile_rect.x,
                                    tile_rect.y,
                                    tile_rect.w,
                                    tile_rect.h,
                                    3.,
                                    color,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[macroquad::main("Airplane Mode")]
async fn main() {
    let mut debug_mode = true;
    let mut input_handler = InputHandler::new();
    set_pc_assets_folder("assets");
    macroquad_profiler::profiler(Default::default());

    let tileset = load_texture("tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("airplane.json").await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();

    let mut player = Player::new();
    // set_fullscreen(true);

    use std::time::Instant;
    let now = Instant::now();

    let map_height = tiled_map.layers.get("main layer").unwrap().height as f32;
    let mut layers = Vec::new();
    for layer in &tiled_map.raw_tiled_map.layers {
        let offsetx = layer.offsetx.unwrap_or_default() as f32;
        let offsety = layer.offsety.unwrap_or_default() as f32;
        let layer = Layer {
            name: layer.name.clone(),
            height: layer.height,
            offsetx,
            offsety,
            ..Default::default()
        };
        layers.push(layer);
    }
    loop {
        clear_background(BLACK);

        player.camera_rect.y = player.rect.y - player.camera_rect.h / 2. - TILE_SIDE;
        let camera = player.camera();
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
        if let Some(c) = get_char_pressed() {
            if c.to_ascii_uppercase() == 'D' {
                debug_mode = !debug_mode;
            }
        }

        let dy = player.speed.y * get_frame_time();

        // check collisions
        if player.rect.y + dy < 0. {
            player.speed.y = 0.;
        }
        if player.rect.y + dy > TILE_SIDE * (map_height - 1.) {
            player.speed.y = 0.;
        }

        // draw map
        for layer in &mut layers {
            layer.update(&tiled_map, &mut player);
            layer.draw(&tiled_map, &mut player, debug_mode);
        }

        {
            player.rect.x += player.speed.x * get_frame_time();
            player.rect.y += player.speed.y * get_frame_time();
        }

        player.draw(&tiled_map, debug_mode);

        set_default_camera();

        #[cfg(target_os = "android")]
        input_handler.draw();

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

        if debug_mode {
            profiler(Default::default());
        }

        next_frame().await;
    }
}
