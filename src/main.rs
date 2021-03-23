mod lib;

use my_snake::{Cell, Collision, Direction, Snake};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::{thread, time};

fn main() {
    let canvas_width = 20_u32;
    let canvas_height = 20_u32;
    let (mut canvas, mut events) = lib::init(canvas_width, canvas_height);
    let mut direction = None;
    let mut snake = Snake::new();
    let mut apple = my_snake::make_apple(&snake, &canvas);
    let mut apples_eaten = 0;
    init_game(
        &mut canvas,
        &mut direction,
        &mut snake,
        &mut apple,
        &mut apples_eaten,
    );
    'game: loop {
        'event_check: for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    timestamp: _timestamp,
                    window_id: _window_id,
                    keycode,
                    scancode: _scancode,
                    keymod: _keymod,
                    repeat: _repeat,
                } => {
                    if keycode == Some(Keycode::Escape) {
                        println!("escaped");
                        break 'game;
                    }
                    let new_dir = to_direction(keycode.unwrap()).unwrap();
                    if direction.is_none() || new_dir != direction.as_ref().unwrap().opposite() {
                        direction = Some(new_dir);
                        break 'event_check;
                    }
                }
                Event::Quit { .. } => {
                    println!("closed");
                    break 'game;
                }
                _ => {}
            }
        }
        snake.move_towards(&direction);
        match snake.check_colision(&apple, &canvas) {
            None => {}
            Some(Collision::Apple) => {
                apples_eaten += 1;
                println!("ate apple #{}", apples_eaten);
                apple = my_snake::make_apple(&snake, &canvas);
                snake.len += 1;
            }
            Some(Collision::Snake) => {
                println!("snake collision");
                init_game(
                    &mut canvas,
                    &mut direction,
                    &mut snake,
                    &mut apple,
                    &mut apples_eaten,
                );
            }
            Some(Collision::Box) => {
                println!("box collision");
                init_game(
                    &mut canvas,
                    &mut direction,
                    &mut snake,
                    &mut apple,
                    &mut apples_eaten,
                );
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        snake.draw(&mut canvas);
        apple.unwrap().draw(&mut canvas);
        canvas.present();
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn to_direction(keycode: Keycode) -> Option<Direction> {
    match keycode {
        Keycode::Right => Some(Direction::Right),
        Keycode::Left => Some(Direction::Left),
        Keycode::Down => Some(Direction::Down),
        Keycode::Up => Some(Direction::Up),
        _ => None,
    }
}

fn init_game(
    canvas: &mut Canvas<Window>,
    direction: &mut Option<Direction>,
    snake: &mut Snake,
    apple: &mut Option<Cell>,
    apples_eaten: &mut i32,
) {
    *direction = None;
    *snake = Snake::new();
    *apple = my_snake::make_apple(&snake, &canvas);
    *apples_eaten = 0;
}
