use crate::*;

#[derive(Clone)]
pub struct Food {
    pub pos: Point,
}

impl Food {
    pub fn gen_foods(g: &mut Game) {
        let food_count = thread_rng().gen_range(1..=MAX_FOOD_COUNT.min(g.free_pos.len() as i16));
        g.foods = Vec::with_capacity(food_count as usize);
        for _i in 0..food_count {
            g.foods.push(Food {
                pos: g.free_pos.pop_random(),
            });
        }
    }
    pub fn render(&self, t: Viewport, gfx: &mut GlGraphics) {
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
