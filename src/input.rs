/// Separate file for android specific code needed

#[cfg(target_os = "android")]
use macroquad::math::Vec2;
use macroquad::prelude::*;
#[cfg(target_os = "android")]
use std::collections::HashMap;

#[cfg(target_os = "android")]
fn draw_segment(x: f32, y: f32, radius: f32, rotation: f32, color: Color) {
    let rot = rotation.to_radians();
    let mut prev = Default::default();
    for i in 0..(5 + 1) {
        let rx = ((i as f32 * std::f32::consts::PI * 2.) / 20. + rot).cos();
        let ry = ((i as f32 * std::f32::consts::PI * 2.) / 20. + rot).sin();

        if i != 0 {
            draw_triangle(
                Vec2::new(x, y),
                prev,
                Vec2::new(x + radius * rx, y + radius * ry),
                if i % 2 == 0 { color } else { MAGENTA },
            );
        }
        prev = Vec2::new(x + radius * rx, y + radius * ry);
    }
}

#[cfg(target_os = "android")]
fn rotate(p: Vec2, c: Vec2, angle: f32) -> Vec2 {
    let angle = angle.to_radians();
    Vec2::new(
        angle.cos() * (p.x - c.x) - angle.sin() * (p.y - c.y) + c.x,
        angle.sin() * (p.x - c.x) + angle.cos() * (p.y - c.y) + c.y,
    )
}

#[derive(Default)]
pub(crate) struct InputHandler {
    // rewrite to process multiple touches
    #[cfg(target_os = "android")]
    touch_id: u64,
    #[cfg(target_os = "android")]
    touch_start: Vec2,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
    #[cfg(target_os = "android")]
    up_touch: bool,
    #[cfg(target_os = "android")]
    right_touch: bool,
    #[cfg(target_os = "android")]
    down_touch: bool,
    #[cfg(target_os = "android")]
    left_touch: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        #[cfg(target_os = "android")]
        for touch in touches().iter().take(1) {
            match touch.phase {
                TouchPhase::Started => {
                    self.touch_start = touch.position;
                }
                _ => (),
            };

            let angle = (self.touch_start.y - touch.position.y)
                .atan2(self.touch_start.x - touch.position.x)
                .to_degrees();
            if self.touch_start != touch.position {
                match angle {
                    x if x >= 0. && x < 45. => {
                        self.left_touch = true;
                    }
                    x if x >= 45. && x < 135. => {
                        self.up_touch = true;
                    }
                    x if x >= 135. && x <= 180. => {
                        self.right_touch = true;
                    }
                    x if x <= -135. && x >= -180. => {
                        self.right_touch = true;
                    }
                    x if x <= -45. && x > -135. => {
                        self.down_touch = true;
                    }
                    x if x < 0. && x > -45. => {
                        self.left_touch = true;
                    }
                    _ => panic!("Wrong angle! How did you even make this?!"),
                }
            }
        }

        self.up = is_key_down(KeyCode::Up);
        self.right = is_key_down(KeyCode::Right);
        self.down = is_key_down(KeyCode::Down);
        self.left = is_key_down(KeyCode::Left);
        #[cfg(target_os = "android")]
        {
            self.up = self.up || self.up_touch;
            self.right = self.right || self.right_touch;
            self.down = self.down || self.down_touch;
            self.left = self.left || self.left_touch;

            self.up_touch = false;
            self.down_touch = false;
            self.left_touch = false;
            self.right_touch = false;
        }
    }

    #[cfg(target_os = "android")]
    pub fn draw(&self) {
        for touch in touches().iter().take(1) {
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => (GREEN, 80.0),
                TouchPhase::Stationary => (RED, 60.0),
                TouchPhase::Moved => (ORANGE, 60.0),
                TouchPhase::Ended => (BLUE, 80.0),

                TouchPhase::Cancelled => (BLACK, 80.0),
            };
            draw_line(
                self.touch_start.x,
                self.touch_start.y,
                touch.position.x,
                touch.position.y,
                2.,
                fill_color,
            );
            let line_end = Vec2::new(self.touch_start.x, self.touch_start.y - size);
            for i in 0..4 {
                let new_end = rotate(line_end, self.touch_start, 90.0 * i as f32 + 45.0);
                draw_line(
                    self.touch_start.x,
                    self.touch_start.y,
                    new_end.x,
                    new_end.y,
                    2.,
                    fill_color,
                );
            }
            let angle = (self.touch_start.y - touch.position.y)
                .atan2(self.touch_start.x - touch.position.x)
                .to_degrees();
            let seg_ang;
            if self.touch_start != touch.position {
                match angle {
                    x if x >= 0. && x < 45. => {
                        seg_ang = 135.;
                    }
                    x if x >= 45. && x < 135. => {
                        seg_ang = 225.;
                    }
                    x if x >= 135. && x <= 180. => {
                        seg_ang = 315.;
                    }
                    x if x <= -135. && x >= -180. => {
                        seg_ang = 315.;
                    }
                    x if x <= -45. && x > -135. => {
                        seg_ang = 45.;
                    }
                    x if x < 0. && x > -45. => {
                        seg_ang = 135.;
                    }
                    _ => panic!("Wrong angle! How did you even make this?!"),
                }
                draw_segment(
                    self.touch_start.x,
                    self.touch_start.y,
                    size,
                    seg_ang,
                    SKYBLUE,
                );
            }
            draw_circle_lines(self.touch_start.x, self.touch_start.y, size, 2., fill_color);
            draw_circle(touch.position.x, touch.position.y, size, fill_color);
            draw_text(
                format!("ANGLE: {}", angle).as_str(),
                10.,
                30.,
                20.,
                DARKGRAY,
            );
        }
    }
}
