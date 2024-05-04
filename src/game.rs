use crate::*;

pub struct Game {
    pub snake: Snake,
    time: f64,
    update_time: f64,
    pub state: State,
    pub foods: Vec<Food>,
    pub free_pos: Bijection<Point>,
}

impl Game {
    pub fn new() -> Game {
        let mut g = Game {
            snake: Snake::new(VecDeque::from(INITIAL_SNAKE), Key::Down),
            time: UPDATE_TIME,
            update_time: UPDATE_TIME,
            state: State::Playing,
            foods: Vec::new(),
            free_pos: Bijection::with_capacity(BOARD_WIDTH as usize * BOARD_HEIGHT as usize),
        };
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                g.free_pos.insert(Point { x, y });
            }
        }
        for pos in INITIAL_SNAKE.iter() {
            g.free_pos.remove_elem(pos);
        }
        Food::gen_foods(&mut g);
        g
    }
    fn restart(&mut self) {
        self.snake = Snake::new(VecDeque::from(INITIAL_SNAKE), Key::Down);
        self.time = UPDATE_TIME;
        self.update_time = UPDATE_TIME;
        self.state = State::Playing;
        self.foods = Vec::new();
        self.free_pos = Bijection::with_capacity(BOARD_WIDTH as usize * BOARD_HEIGHT as usize);
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                self.free_pos.insert(Point { x, y });
            }
        }
        for pos in INITIAL_SNAKE.iter() {
            self.free_pos.remove_elem(pos);
        }
        Food::gen_foods(self);
    }
    fn draw_board(t: Viewport, gfx: &mut GlGraphics) {
        gfx.clear_color(color::hex(BG_COLOR_ODD));

        gfx.draw(t, |_a, b| {
            for x in 0..BOARD_WIDTH {
                for y in (x % 2..BOARD_HEIGHT).step_by(2) {
                    rectangle(
                        color::hex(BG_COLOR_EVEN),
                        (
                            x as f64 * TILE_SIZE as f64,
                            y as f64 * TILE_SIZE as f64,
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
    pub fn render(&mut self, t: Viewport, gfx: &mut GlGraphics) {
        Self::draw_board(t, gfx);
        self.snake.render(t, gfx);

        for food in self.foods.iter() {
            food.render(t, gfx);
        }
    }
    pub fn update(&mut self, dt: f64) {
        if self.state == State::Playing {
            self.time += dt;

            if self.time > self.update_time {
                self.time -= self.update_time;
                Snake::update(self);
            }
        }
    }
    pub fn key_press(&mut self, key: Key) {
        match key {
            Key::R => self.restart(),
            Key::P => {
                self.state = match self.state {
                    State::Paused => State::Playing,
                    State::Playing => State::Paused,
                    other => other,
                }
            }
            Key::Left | Key::Right | Key::Up | Key::Down => self.snake.key_press(key),
            _ => {}
        }
    }
}
