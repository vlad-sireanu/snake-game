use crate::*;

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
                            + if prev.x < p.x { -1.0 } else { 1.0 } * MARGIN_SIZE as f64,
                        p.y as f64 * TILE_SIZE as f64
                            + if prev.y < p.y { -1.0 } else { 1.0 } * MARGIN_SIZE as f64,
                        TILE_SIZE as f64
                            + if prev.x != p.x { 0.0 } else { -2.0 } * MARGIN_SIZE as f64,
                        TILE_SIZE as f64
                            + if prev.y != p.y { 0.0 } else { -2.0 } * MARGIN_SIZE as f64,
                    ),
                    t.abs_transform(),
                    b,
                );
                prev = p;
            }
        });
    }
    pub fn key_press(&mut self, k: Key) {
        if Self::opposite_direction(k) != self.last_pressed {
            self.keys.push_back(k);
            self.last_pressed = k;
        }
    }
    pub fn update(g: &mut Game) {
        use piston::input::keyboard::Key::*;
        let k = if g.snake.keys.is_empty() {
            g.snake.last_pressed
        } else {
            g.snake.keys.pop_front().unwrap()
        };
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
        let pos = *g.snake.tail.front().unwrap() + dxy;

        g.snake.tail.push_front(pos);

        for (index, food) in g.foods.iter().enumerate() {
            if food.pos == pos {
                g.foods.swap_remove(index);

                if g.snake.tail.len() as i16 == BOARD_WIDTH * BOARD_HEIGHT {
                    g.state = State::GameOver;
                    println!("You won!\nScore: {}\n", g.snake.tail.len());
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
            println!("You died!\nScore: {}\n", g.snake.tail.len());
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
