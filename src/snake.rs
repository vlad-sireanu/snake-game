use crate::food::Food;
use crate::game::Game;

use crate::*;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::keyboard::Key;
use std::collections::VecDeque;

pub struct Snake {
    pub tail: VecDeque<Point>,
    keys: VecDeque<Key>,
    last_pressed: Key,
}

impl Snake {
    pub fn new(tail: VecDeque<Point>, key: Key) -> Snake {
        Snake {
            tail: tail,
            keys: VecDeque::new(),
            last_pressed: key,
        }
    }
    pub fn render(&self, t: Viewport, gfx: &mut GlGraphics) {
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
    pub fn key_press(&mut self, k: Key) {
        use piston::input::keyboard::Key::*;
        match k {
            Right | Down | Left | Up if Self::opposite_direction(k) != self.last_pressed => {
                self.keys.push_back(k);
                self.last_pressed = k;
            }
            _ => {}
        };
    }
    pub fn update(g: &mut Game) {
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

        g.snake.tail.push_front(pos);

        for (index, food) in g.foods.iter().enumerate() {
            if food.pos == pos {
                g.foods.swap_remove(index);
                g.score += 1;

                if g.score == BOARD_WIDTH * BOARD_HEIGHT {
                    g.state = State::GameOver;
                    println!("You won!\nScore: {}\n", g.score);
                    return;
                }

                if g.foods.len() == 0 {
                    Food::gen_foods(g);
                }
                return;
            }
        }

        if Self::outside(pos) || !g.free_pos.contains(pos) {
            g.state = State::GameOver;
            println!("You died!\nScore: {}\n", g.score);
            return;
        }

        g.free_pos.remove_elem(&pos);
        g.free_pos.insert(g.snake.tail.pop_back().unwrap());
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
