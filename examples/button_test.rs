extern crate piston_window;
extern crate button_controller;

use piston_window::*;
use button_controller::{ButtonController, ButtonEvent, ButtonState};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("button", [300; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut click_me = ButtonController::new();
    let click_me_layout = [10.0, 60.0, 280.0, 180.0];
    while let Some(e) = window.next() {
        click_me.event(click_me_layout, math::identity(), &e);
        for e in &click_me.events {
            println!("{}", match *e {
                ButtonEvent::Click => "Click",
                ButtonEvent::MouseEnter => "MouseEnter",
                ButtonEvent::MouseLeave => "MouseLeave",
                ButtonEvent::Press => "Press",
                ButtonEvent::Cancel => "Cancel",
            })
        }

        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            let color = match click_me.state(0.2) {
                ButtonState::Inactive => [0.6, 0.6, 0.6, 1.0],
                ButtonState::Hover => [0.4, 0.4, 0.4, 1.0],
                ButtonState::Press => [0.1, 0.1, 0.1, 1.0],
                ButtonState::Cancel => [0.3, 0.2, 0.2, 1.0],
            };
            rectangle(color, click_me_layout, c.transform, g);
        });
    }
}
