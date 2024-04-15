use std::f32::consts::PI;

use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{
    map::{Map, TileType},
    player::Player,
};

const PI2: f32 = PI / 2.0;
const PI3: f32 = 3.0 * PI / 2.0;
const DEGREE_RAD: f32 = 0.0174533;

pub struct Ray {
    p1: Point,
    p2: Point,
    angle: f32,
    distance: f32,
    color: Color,
}

impl Ray {
    pub fn new(player: &Player, map: &Map, qtd: usize) -> Vec<Ray> {
        let mut rays = Vec::with_capacity(qtd);
        let mut angle = player.angle;

        angle = add_angle(angle, -30.0 * DEGREE_RAD);
        let player_x = player.x;
        let player_y = player.y;
        let map_x = map.x();
        let mut color = Color::RGB(0, 0, 255);
        let map_y = map.y();
        let mut r_distance;

        for _ in 0..qtd {
            let angle_tan = angle.tan();
            let inverse_angle_tan = -1.0 / angle_tan;
            let negative_angle_tan = -angle_tan;
            let mut x = 0.0;
            let mut y = 0.0;

            let mut horizontal_distance = f32::MAX;
            let mut horizontal_x = player.x;
            let mut horizontal_y = player.y;

            let mut vertical_distance = f32::MAX;
            let mut vertical_x = player.x;
            let mut vertical_y = player.y;

            //Horizontal
            {
                let mut y_offset = 0.0;
                let mut x_offset = 0.0;
                let mut distance_offset = 0;
                if angle > PI {
                    //loking up
                    y = ((player_y as i32 >> 6) << 6) as f32 - 0.0001;
                    x = (player_y - y) * inverse_angle_tan + player.x;
                    y_offset = -64.0;
                    x_offset = -y_offset * inverse_angle_tan;
                } else if angle < PI {
                    //loking down
                    y = ((player_y as i32 >> 6) << 6) as f32 + 64.0;
                    x = (player_y - y) * inverse_angle_tan + player.x;
                    y_offset = 64.0;
                    x_offset = -y_offset * inverse_angle_tan;
                } else if angle == 0.0 || angle == PI {
                    //loking right or left
                    x = player.x;
                    y = player.y;
                    distance_offset = map_x
                }

                while distance_offset < map_x {
                    let max_x = x as i32 >> 6;
                    let max_y = y as i32 >> 6;

                    let position = max_y * map_x as i32 + max_x;
                    if position < 0 || position as usize >= map.size() {
                        horizontal_x = x;
                        horizontal_y = y;
                        horizontal_distance = distance(player_x, player_y, x, y);
                        break;
                    }
                    if let Some(tile) = map.tile_at(position as usize) {
                        if tile.tile_type == TileType::Wall {
                            color = tile.color;
                            horizontal_x = x;
                            horizontal_y = y;
                            horizontal_distance = distance(player_x, player_y, x, y);
                            break;
                        } else {
                            x += x_offset;
                            y += y_offset;
                        }
                    } else {
                        x += x_offset;
                        y += y_offset;
                    }
                }
            }

            {
                //Vertical
                let mut y_offset = 0.0;
                let mut x_offset = 0.0;
                let mut distance_offset = 0;

                if angle > PI2 && angle < PI3 {
                    //loking left
                    x = ((player_x as i32 >> 6) << 6) as f32 - 0.0001;
                    y = (player_x - x) * negative_angle_tan + player_y;
                    x_offset = -64.0;
                    y_offset = -x_offset * negative_angle_tan;
                } else if angle < PI2 || angle > PI3 {
                    //loking right
                    x = ((player_x as i32 >> 6) << 6) as f32 + 64.0;
                    y = (player_x - x) * negative_angle_tan + player_y;
                    x_offset = 64.0;
                    y_offset = -x_offset * negative_angle_tan;
                } else if angle == 0.0 || angle == PI {
                    //loking up or down
                    x = player.x;
                    y = player.y;
                    distance_offset = map_y;
                }

                while distance_offset < map_y {
                    let max_x = x as i32 >> 6;
                    let max_y = y as i32 >> 6;

                    let position = max_y * map_x as i32 + max_x;
                    if position < 0 || position as usize >= map.size() {
                        vertical_x = x;
                        vertical_y = y;
                        vertical_distance = distance(player_x, player_y, x, y);
                        break;
                    }
                    if let Some(tile) = map.tile_at(position as usize) {
                        if tile.tile_type == TileType::Wall {
                            color = tile.color;
                            vertical_x = x;
                            vertical_y = y;
                            vertical_distance = distance(player_x, player_y, x, y);
                            break;
                        } else {
                            x += x_offset;
                            y += y_offset;
                        }
                    } else {
                        x += x_offset;
                        y += y_offset;
                    }
                }
            }

            if horizontal_distance < vertical_distance {
                x = horizontal_x;
                y = horizontal_y;
                r_distance = horizontal_distance;
            } else {
                x = vertical_x;
                y = vertical_y;
                r_distance = vertical_distance;
                color = darker_color(color);
            }

            let p1 = Point::new(player.x as i32, player.y as i32);

            if x < -20589.0 {
                x = -20589.0;
            } else if x > 20589.0 {
                x = 20589.0;
            }

            if y < -20589.0 {
                y = -20589.0;
            } else if y > 20589.0 {
                y = 20589.0;
            }

            let p2 = Point::new(x as i32, y as i32);
            rays.push(Ray {
                p1,
                p2,
                angle,
                distance: r_distance,
                color,
            });

            angle = add_angle(angle, DEGREE_RAD);
        }
        return rays;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        let _ = canvas.draw_line(self.p1, self.p2);
    }

    pub fn draw_3d(&self, mut i: usize, canvas: &mut Canvas<Window>, map: &Map, player: &Player) {
        let map_size = map.size();
        let mut d_angle = player.angle - self.angle;
        if d_angle < 0.0 {
            d_angle += 2.0 * PI;
        } else if d_angle > 2.0 * PI {
            d_angle -= 2.0 * PI;
        }
        let distance = self.distance * d_angle.cos();
        let mut wall_height = (map_size * 320) as f32 / distance;
        if wall_height > 320.0 {
            wall_height = 320.0;
        }
        let wall_offset = (160.0 - wall_height / 2.0) as i32;

        canvas.set_draw_color(self.color);

        //I couldn't figure out how to make with Rectangles so i used 8 lines to make each wall segment
        i = i * 8;
        for y in 0..=8 {
            let x = i + y;
            let x = (x + 530) as i32;
            let p1 = Point::new(x, wall_offset);
            let p2 = Point::new(x, wall_height as i32 + wall_offset);
            let _ = canvas.draw_line(p1, p2);
        }
    }
}

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn add_angle(mut target: f32, num: f32) -> f32 {
    target += num;
    if target > 2.0 * PI {
        target -= 2.0 * PI;
    } else if target < 0.0 {
        target += 2.0 * PI;
    }
    target
}

fn darker_color(color: Color) -> Color {
    let r = color.r as f32 * 0.5;
    let g = color.g as f32 * 0.5;
    let b = color.b as f32 * 0.5;
    Color::RGB(r as u8, g as u8, b as u8)
}
