use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

const RESOLUTION: i32 = 32;

#[derive(Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Cell {
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        let r = Rect::new(
            self.x * RESOLUTION,
            self.y * RESOLUTION,
            RESOLUTION as u32,
            RESOLUTION as u32,
        );
        canvas.fill_rect(r);
    }
}

pub struct Snake {
    pub body: Vec<Cell>,
    pub len: usize,
}

pub enum Collision {
    Apple,
    Snake,
    Box,
}

impl Snake {
    pub fn check_colision(
        &self,
        apple: &Option<Cell>,
        canvas: &Canvas<Window>,
    ) -> Option<Collision> {
        // check self
        let head = self.body.first().unwrap();
        for c in self.body.iter().skip(1) {
            if head.x == c.x && head.y == c.y {
                return Some(Collision::Snake);
            }
        }
        // check apple
        if apple.is_some() {
            let apple = apple.unwrap();
            if apple.x == head.x && apple.y == head.y {
                return Some(Collision::Apple);
            }
        }
        // check box
        if head.x < 0
            || head.x >= canvas.window().size().0 as i32 / RESOLUTION
            || head.y < 0
            || head.y >= canvas.window().size().1 as i32 / RESOLUTION
        {
            return Some(Collision::Box);
        }
        None
    }
}

pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "Snake",
            width * RESOLUTION as u32,
            height * RESOLUTION as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().present_vsync().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

impl Snake {
    pub fn move_towards(&mut self, direction: &Option<Direction>) {
        let mut new = *(self.body.first().unwrap());
        match direction {
            None => {
                return;
            }
            Some(direction) => match direction {
                Direction::Up => new.y -= 1,
                Direction::Down => new.y += 1,
                Direction::Right => new.x += 1,
                Direction::Left => new.x -= 1,
            },
        }
        self.body.insert(0, new);
        self.body.truncate(self.len);
    }
}

impl Snake {
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for c in self.body.iter() {
            c.draw(canvas);
        }
    }
}

impl Snake {
    pub fn new() -> Snake {
        let v = vec![Cell {
            x: 3,
            y: 3,
            color: Color {
                r: 0,
                g: 255,
                b: 0,
                a: 0,
            },
        }];
        Snake { body: v, len: 1 }
    }
}

pub fn make_apple(snake: &Snake, canvas: &Canvas<Window>) -> Option<Cell> {
    let mut rng = rand::thread_rng();
    loop {
        let apple = Cell {
            x: rng.gen_range(0..(canvas.window().size().0 / RESOLUTION as u32)) as i32,
            y: rng.gen_range(0..(canvas.window().size().1 / RESOLUTION as u32)) as i32,
            color: Color {
                r: 255,
                g: 0,
                b: 0,
                a: 0,
            },
        };
        let mut collision = false;
        for c in snake.body.iter() {
            if c.x == apple.x && c.y == apple.y {
                collision = true;
                break;
            }
        }
        if !collision {
            return Some(apple);
        }
    }
}
