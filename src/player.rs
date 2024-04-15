use std::f32::consts::PI;

use sdl2::{
    keyboard::Scancode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::map::Map;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub angle: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        let angle: f32 = 0.0;
        Player {
            x,
            y,
            delta_x: angle.cos() * 5.0,
            delta_y: angle.sin() * 5.0,
            angle: 0.0,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));

        let _ = canvas.fill_rect(self.react());
        let (p1, p2) = self.angle_line();
        let _ = canvas.draw_line(p1, p2);
    }

    pub fn update(&mut self, keycode: Scancode, map: &Map) {
        let mut new_x = self.x;
        let mut new_y = self.y;
        let mut new_angle = self.angle;
        let mut new_delta_x = self.delta_x;
        let mut new_delta_y = self.delta_y;

        match keycode {
            Scancode::A => {
                new_angle = self.angle - 0.1;
                if new_angle < 0.0 {
                    new_angle = 2.0 * PI - 0.1;
                }
                new_delta_x = new_angle.cos() * 5.0;
                new_delta_y = new_angle.sin() * 5.0;
            }
            Scancode::D => {
                new_angle = self.angle + 0.1;
                if new_angle >= 2.0 * PI {
                    new_angle = 0.1;
                }
                new_delta_x = new_angle.cos() * 5.0;
                new_delta_y = new_angle.sin() * 5.0;
            }
            Scancode::W => {
                new_x += new_delta_x;
                new_y += new_delta_y;
            }
            Scancode::S => {
                new_x -= new_delta_x;
                new_y -= new_delta_y;
            }

            _ => {}
        }
        let react = Rect::new(new_x as i32, new_y as i32, 1, 1);
        if !map.is_wall(react) {
            self.x = new_x;
            self.y = new_y;
            self.angle = new_angle;
            self.delta_x = new_delta_x;
            self.delta_y = new_delta_y;
        }
    }

    fn react(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, 2, 2)
    }

    fn angle_line(&self) -> (Point, Point) {
        let p1 = Point::new(self.x as i32, self.y as i32);
        let p2 = Point::new(
            (self.x + self.delta_x * 2.0) as i32,
            (self.y + self.delta_y * 2.0) as i32,
        );
        (p1, p2)
    }
}
