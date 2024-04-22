extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

use glutin_window::GlutinWindow;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::keyboard::Key;
use piston::input::{Button, RenderEvent};
use piston::{PressEvent, UpdateEvent};
use piston_window::WindowSettings;
use rand::Rng;
use std::collections::VecDeque;

const BOARD_WIDTH: i8 = 12;
const BOARD_HEIGHT: i8 = 12;
const TILE_SIZE: i8 = 34;
const MARGIN_SIZE: i8 = 7;
const UPDATE_TIME: f64 = 0.15;
const SNAKE_COLOR: &str = "4444ff";
const FOOD_COLOR: &str = "ff0055";
const BG_COLOR_EVEN: &str = "a9ed81";
const BG_COLOR_ODD: &str = "cafe8a";

const INITIAL_SNAKE: [Point; 3] = [
    Point { x: 2, y: 1 },
    Point { x: 2, y: 0 },
    Point { x: 2, y: -1 },
];
const INITIAL_FOOD: Point = Point { x: 3, y: 3 };

#[derive(PartialEq, Copy, Clone)]
enum State {
    Playing,
    Paused,
    GameOver,
}

#[derive(PartialEq, Copy, Clone)]
struct Point {
    x: i8,
    y: i8,
}

struct Snake {
    tail: VecDeque<Point>,
    keys: VecDeque<Key>,
    last_pressed: Key,
}

impl Snake {
    fn new(tail: VecDeque<Point>, key: Key) -> Snake {
        Snake {
            tail: tail,
            keys: VecDeque::new(),
            last_pressed: key,
        }
    }

    fn render(&self, t: Viewport, gfx: &mut GlGraphics) {
        gfx.draw(t, |_a, b| {
            let mut prev: &Point = self.tail.front().unwrap();
            for p in self.tail.iter() {
                rectangle(
                    color::hex(SNAKE_COLOR),
                    (
                        p.x as f64 * TILE_SIZE as f64
                            + MARGIN_SIZE as f64 * (1.0 - (prev.x < p.x) as i8 as f64 * 2.0),
                        p.y as f64 * TILE_SIZE as f64
                            + MARGIN_SIZE as f64 * (1.0 - (prev.y < p.y) as i8 as f64 * 2.0),
                        TILE_SIZE as f64
                            + MARGIN_SIZE as f64
                                * (-2.0 + (prev.x < p.x || prev.x > p.x) as i8 as f64 * 2.0),
                        TILE_SIZE as f64
                            + MARGIN_SIZE as f64
                                * (-2.0 + (prev.y < p.y || prev.y > p.y) as i8 as f64 * 2.0),
                    ),
                    t.abs_transform(),
                    b,
                );
                prev = p;
            }
        });
    }

    fn key_press(&mut self, k: Key) {
        use piston::input::keyboard::Key::*;
        match k {
            Right | Down | Left | Up if Self::opposite_direction(k) != self.last_pressed => {
                self.keys.push_back(k);
                self.last_pressed = k;
            }
            _ => {}
        };
    }

    fn update(g: &mut Game) {
        use piston::input::keyboard::Key::*;
        if g.snake.keys.is_empty() {
            g.snake.keys.push_back(g.snake.last_pressed);
        }
        let k = g.snake.keys.pop_front().unwrap();
        Snake::mv(
            g,
            match k {
                Right => Point { x: 1, y: 0 },
                Down => Point { x: 0, y: 1 },
                Left => Point { x: -1, y: 0 },
                Up => Point { x: 0, y: -1 },
                _ => panic!("only arrows allowed"),
            },
        )
    }

    fn mv(g: &mut Game, dxy: Point) {
        let pos = Point {
            x: g.snake.tail.front().unwrap().x + dxy.x,
            y: g.snake.tail.front().unwrap().y + dxy.y,
        };

        if Self::outside(pos) || g.snake.collides(pos) {
            g.snake.tail.push_front(pos);
            g.state = State::GameOver;
            println!("You died!\nScore: {}\n", g.score);
            return;
        }

        g.snake.tail.push_front(pos);

        if g.food.pos == pos {
            g.score += 1;
            if g.score == BOARD_WIDTH as u32 * BOARD_HEIGHT as u32 {
                g.state = State::GameOver;
                println!("You won!\nScore: {}\n", g.score);
                return;
            }

            g.food = Food::new(Food::gen_pos(g));
            return;
        }

        g.snake.tail.pop_back();
    }

    fn collides(&self, pos: Point) -> bool {
        self.tail.iter().any(|t| *t == pos)
    }

    fn opposite_direction(key: Key) -> Key {
        use piston::input::keyboard::Key::*;
        match key {
            Right => Left,
            Left => Right,
            Up => Down,
            Down => Up,
            other => other,
        }
    }

    fn outside(p: Point) -> bool {
        p.x < 0 || BOARD_WIDTH <= p.x || p.y < 0 || BOARD_HEIGHT <= p.y
    }
}

struct Food {
    pos: Point,
}

impl Food {
    fn new(pos: Point) -> Food {
        Food { pos: pos }
    }

    fn gen_pos(g: &Game) -> Point {
        loop {
            let mut rng = rand::thread_rng();
            let pos = Point {
                x: rng.gen_range(0..BOARD_WIDTH),
                y: rng.gen_range(0..BOARD_HEIGHT),
            };

            if !g.snake.collides(pos) {
                return pos;
            }
        }
    }

    fn render(&self, t: Viewport, gfx: &mut GlGraphics) {
        gfx.draw(t, |_a, b| {
            rectangle(
                color::hex(FOOD_COLOR),
                rectangle::square(
                    self.pos.x as f64 * TILE_SIZE as f64 + MARGIN_SIZE as f64,
                    self.pos.y as f64 * TILE_SIZE as f64 + MARGIN_SIZE as f64,
                    TILE_SIZE as f64 - 2.0 * MARGIN_SIZE as f64,
                ),
                t.abs_transform(),
                b,
            );
        });
    }
}

struct Game {
    snake: Snake,
    time: f64,
    update_time: f64,
    state: State,
    food: Food,
    score: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            snake: Snake::new(VecDeque::from(INITIAL_SNAKE), Key::Down),
            time: UPDATE_TIME,
            update_time: UPDATE_TIME,
            state: State::Playing,
            score: INITIAL_SNAKE.len() as u32,
            food: Food::new(INITIAL_FOOD),
        }
    }

    fn draw_board(t: Viewport, gfx: &mut GlGraphics) {
        gfx.draw(t, |_a, b| {
            for i in 0..BOARD_HEIGHT {
                for j in 0..BOARD_WIDTH {
                    rectangle(
                        if (i + j) % 2 == 0 {
                            color::hex(BG_COLOR_EVEN)
                        } else {
                            color::hex(BG_COLOR_ODD)
                        },
                        (
                            i as f64 * TILE_SIZE as f64,
                            j as f64 * TILE_SIZE as f64,
                            TILE_SIZE as f64,
                            TILE_SIZE as f64,
                        ),
                        t.abs_transform(),
                        b,
                    );
                }
            }
        });
    }

    fn render(&mut self, t: Viewport, gfx: &mut GlGraphics) {
        // clear(color::hex(BG_COLOR), gfx);
        Self::draw_board(t, gfx);
        self.food.render(t, gfx);
        self.snake.render(t, gfx);
    }

    fn update(&mut self, dt: f64) {
        match self.state {
            State::Paused | State::GameOver => return,
            _ => {}
        }

        self.time += dt;

        if self.time > self.update_time {
            self.time -= self.update_time;
            Snake::update(self);
        }
    }

    fn key_press(&mut self, key: Key) {
        match (key, self.state) {
            (Key::R, _) => {
                self.snake = Snake::new(VecDeque::from(INITIAL_SNAKE), Key::Down);
                self.state = State::Playing;
                self.time = UPDATE_TIME;
                self.update_time = UPDATE_TIME;
                self.state = State::Playing;
                self.score = INITIAL_SNAKE.len() as u32;
                self.food = Food::new(Food::gen_pos(self));
            }
            (Key::P, State::Playing) => {
                self.state = State::Paused;
            }
            (Key::P, State::Paused) => {
                self.state = State::Playing;
            }
            _ => {
                self.snake.key_press(key);
            }
        };
    }
}

fn main() {
    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [
            BOARD_WIDTH as u32 * TILE_SIZE as u32,
            BOARD_HEIGHT as u32 * TILE_SIZE as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .expect("window failed");
    let mut gfx = GlGraphics::new(OpenGL::V3_2);
    let mut events = Events::new(EventSettings::new());
    let mut game = Game::new();

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            let t = args.viewport();
            game.render(t, &mut gfx);
        }
        if let Some(button) = e.press_args() {
            if let Button::Keyboard(key) = button {
                game.key_press(key);
            }
        }
        if let Some(args) = e.update_args() {
            game.update(args.dt);
        }
    }
}
