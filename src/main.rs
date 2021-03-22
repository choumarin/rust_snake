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
    let mut direction = Direction::None;
    let mut snake = Snake::new();
    let mut apple = my_snake::make_apple(&snake, &canvas);

    init_game(&mut canvas, &mut direction, &mut snake, &mut apple);
    'game: loop {
        let mut key_pressed = false;
        'event_check: for event in events.poll_iter() {
            if event.is_keyboard() {
                // avoid processing more than 1 key
                if (!key_pressed) {
                    key_pressed = true;
                } else {
                    continue 'event_check;
                }
            }
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if direction != Direction::Down {
                        direction = Direction::Up;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if direction != Direction::Up {
                        direction = Direction::Down;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if direction != Direction::Right {
                        direction = Direction::Left;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if direction != Direction::Left {
                        direction = Direction::Right;
                    }
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("closed");
                    break 'game;
                }
                _ => continue 'game,
            }
        }
        snake.move_towards(&direction);
        match snake.check_colision(&apple, &canvas) {
            None => {}
            Some(Collision::Apple) => {
                println!("ate apple");
                apple = my_snake::make_apple(&snake, &canvas);
                snake.len = snake.len + 1;
            }
            Some(Collision::Snake) => {
                println!("snake collision");
                init_game(&mut canvas, &mut direction, &mut snake, &mut apple);
            }
            Some(Collision::Box) => {
                println!("box collision");
                init_game(&mut canvas, &mut direction, &mut snake, &mut apple);
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

fn init_game(
    canvas: &mut Canvas<Window>,
    direction: &mut Direction,
    snake: &mut Snake,
    apple: &mut Option<Cell>,
) -> () {
    *direction = Direction::None;
    *snake = Snake::new();
    *apple = my_snake::make_apple(&snake, &canvas);
}
