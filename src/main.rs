extern crate piston_window;

mod game;

use game::{to_coordinate, GamePosition, SCREEN_WIDTH};
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Move Rectangle",
        [
            to_coordinate(SCREEN_WIDTH),
            to_coordinate(game::SCREEN_HEIGHT),
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut gm_postion: GamePosition = GamePosition::new(
        0.0,
        0.0,
        to_coordinate(game::SCREEN_HEIGHT / 6),
        to_coordinate(SCREEN_WIDTH / 6),
    );

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            gm_postion.check_key_event(key);
        }

        event.update(|arg| {
            gm_postion.update_time(arg.dt);
        });

        window.draw_2d(&event, |c, g, _| {
            gm_postion.draw(&c, g);
        });
    }
}
