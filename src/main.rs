use snake_game::*;

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
