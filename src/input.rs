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
    for i in 0..(2 + 1) {
        let rx = ((i as f32 * std::f32::consts::PI * 2.) / 20. + rot).cos();
        let ry = ((i as f32 * std::f32::consts::PI * 2.) / 20. + rot).sin();

        if i != 0 {
            draw_triangle(
                Vec2::new(x, y),
                prev,
                Vec2::new(x + radius * rx, y + radius * ry),
                color,
            );
        }
        prev = Vec2::new(x + radius * rx, y + radius * ry);
    }
    let rx = ((2.5 * std::f32::consts::PI * 2.) / 20. + rot).cos();
    let ry = ((2.5 * std::f32::consts::PI * 2.) / 20. + rot).sin();

    draw_triangle(
        Vec2::new(x, y),
        prev,
        Vec2::new(x + radius * rx, y + radius * ry),
        color,
    );
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
                self.left_touch = angle >= -67.5 && angle < 67.5;
                self.up_touch = angle >= 22.5 && angle < 157.5;
                self.right_touch =
                    angle >= 112.5 && angle <= 180. || angle >= -180. && angle <= -112.5;
                self.down_touch = angle > -157.5 && angle <= -22.5;
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
                TouchPhase::Stationary => (SKYBLUE, 60.0),
                TouchPhase::Moved => (BLUE, 60.0),
                TouchPhase::Ended => (RED, 80.0),

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
            for i in 0..8 {
                let new_end = rotate(line_end, self.touch_start, 45.0 * i as f32 + 22.5);
                draw_line(
                    self.touch_start.x,
                    self.touch_start.y,
                    new_end.x,
                    new_end.y,
                    2.,
                    fill_color,
                );
            }
            let mut seg_ang = 0.;
            if self.touch_start != touch.position {
                if self.left {
                    seg_ang = if self.up {
                        202.5
                    } else if self.down {
                        112.5
                    } else {
                        157.5
                    };
                } else if self.up {
                    seg_ang = if self.right { 293.5 } else { 247.5 };
                } else if self.right {
                    seg_ang = if self.down { 22.5 } else { 337.5 };
                } else if self.down {
                    seg_ang = 67.5;
                }
                draw_segment(self.touch_start.x, self.touch_start.y, size, seg_ang, fill_color);
            }
            draw_circle_lines(self.touch_start.x, self.touch_start.y, size, 2., fill_color);
            draw_circle(touch.position.x, touch.position.y, size, fill_color);

            let font_size = 30.;
            let text = format!(
                "{}{}{}{}",
                if self.left { 'L' } else { 'X' },
                if self.up { 'U' } else { 'X' },
                if self.right { 'R' } else { 'X' },
                if self.down { 'D' } else { 'X' }
            );
            let text_size = measure_text(&text, None, font_size as _, 1.0);
            draw_text(&text, 6., text_size.height + 1., font_size, DARKGRAY);
        }
    }
}
