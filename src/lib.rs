mod bijection;
mod food;
mod game;
mod snake;

use crate::bijection::Bijection;
use crate::food::Food;
pub use crate::game::Game;
use crate::snake::Snake;

pub use glutin_window::GlutinWindow;
pub use graphics::*;
pub use opengl_graphics::{GlGraphics, OpenGL};
pub use piston::event_loop::{EventSettings, Events};
use piston::input::keyboard::Key;
pub use piston::input::{Button, RenderEvent};
pub use piston::{PressEvent, UpdateEvent};
pub use piston_window::WindowSettings;
use rand::Rng;
use std::collections::VecDeque;

pub const BOARD_WIDTH: i16 = 16;
pub const BOARD_HEIGHT: i16 = 12;
pub const TILE_SIZE: i16 = 34;
const MARGIN_SIZE: i16 = 7;
const MAX_FOOD_COUNT: i16 = 6;
const UPDATE_TIME: f64 = 0.15;
const FOOD_COLOR: &str = "ff0055";
const SNAKE_COLOR: &str = "0000ff";
const BG_COLOR_EVEN: &str = "a9ed81";
const BG_COLOR_ODD: &str = "cafe8a";
const INITIAL_SNAKE: [Point; 3] = [
    Point { x: 2, y: 2 },
    Point { x: 2, y: 1 },
    Point { x: 2, y: 0 },
];

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Playing,
    Paused,
    GameOver,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    x: i16,
    y: i16,
}
