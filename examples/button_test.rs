extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate button_controller;

use piston::*;
use opengl_graphics::*;
use graphics::*;
use sdl2_window::Sdl2Window;
use button_controller::{ButtonController, ButtonEvent, ButtonState};

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new("button", [300; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);
    let mut click_me = ButtonController::new();
    let click_me_layout = [10.0, 60.0, 280.0, 180.0];
    while let Some(e) = events.next(&mut window) {
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

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
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
}
