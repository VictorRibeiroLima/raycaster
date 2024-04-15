use std::time::Duration;

use map::Map;
use ray::Ray;
use sdl2::{event::Event, keyboard::Scancode, pixels::Color, render::Canvas, video::Window, Sdl};

mod map;
mod player;
mod ray;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut canvas = setup_canvas(&sdl);
    let mut event_pump = sdl.event_pump().unwrap();

    let titles = vec![
        1, 1, 1, 1, 1, 1, 1, 1, //
        1, 0, 0, 0, 0, 1, 0, 1, //
        1, 0, 1, 0, 0, 1, 0, 1, //
        1, 0, 0, 0, 0, 1, 0, 1, //
        1, 0, 0, 1, 0, 0, 0, 1, //
        1, 1, 1, 0, 0, 0, 0, 1, //
        1, 0, 0, 0, 0, 0, 0, 1, //
        1, 1, 1, 1, 1, 1, 1, 1, //
    ];

    let map = Map::new(8, 8, titles);
    let mut player = player::Player::new(100.0, 100.0);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        map.draw(&mut canvas);
        player.draw(&mut canvas);
        let rays = Ray::new(&player, &map, 60);
        for (i, ray) in rays.into_iter().enumerate() {
            ray.draw(&mut canvas);
            ray.draw_3d(i, &mut canvas, &map, &player);
        }

        for event in event_pump.keyboard_state().pressed_scancodes() {
            player.update(event, &map);
            match event {
                Scancode::Escape => break 'running,
                _ => {}
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn setup_canvas(sdl: &Sdl) -> Canvas<Window> {
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("raycaster", 1024, 512)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    canvas.clear();
    canvas.present();
    canvas
}
