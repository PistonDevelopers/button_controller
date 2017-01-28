//! A Piston library for handling button state and events

#![deny(missing_docs)]

extern crate vecmath;
extern crate input;

use std::time::Instant;

use math::{Matrix2d, Rectangle};
use input::GenericEvent;

pub mod math;

/// Button event signals.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ButtonEvent {
    /// Mouse entered button.
    MouseEnter,
    /// Mouse leaved button.
    MouseLeave,
    /// Button was pressed.
    Press,
    /// The button was clicked.
    Click,
    /// The button press was canceled.
    Cancel,
}

/// Stores state and events of a button.
pub struct ButtonController {
    /// Whether mouse cursor is inside button.
    pub mouse_cursor_inside: bool,
    /// Whether mouse cursor was inside.
    pub was_inside: bool,
    /// Whether button is pressed.
    pub pressed: bool,
    /// When button was pressed.
    pub pressed_instant: Option<Instant>,
    /// Stores button events.
    pub events: Vec<ButtonEvent>,
}

impl ButtonController {
    /// Creates a new button controller.
    pub fn new() -> ButtonController {
        ButtonController {
            mouse_cursor_inside: false,
            was_inside: false,
            pressed: false,
            pressed_instant: None,
            events: vec![],
        }
    }

    /// Handles event.
    pub fn event<E: GenericEvent>(&mut self, layout: Rectangle, transform: Matrix2d, e: &E) {
        use math::is_inside;
        use input::MouseButton;

        if let Some(pos) = e.mouse_cursor_args() {
            let inside = is_inside(pos, transform, layout);
            if inside {
                if !self.mouse_cursor_inside {
                    self.mouse_cursor_inside = true;
                    self.events.push(ButtonEvent::MouseEnter);
                }
            } else {
                if self.mouse_cursor_inside {
                    self.mouse_cursor_inside = false;
                    self.events.push(ButtonEvent::MouseLeave);
                }
            }
        }

        if let Some(input::Button::Mouse(MouseButton::Left)) = e.press_args() {
            if self.mouse_cursor_inside {
                self.pressed = true;
                self.was_inside = true;
                self.events.push(ButtonEvent::Press);
                self.pressed_instant = Some(Instant::now());
            }
        }
        if let Some(input::Button::Mouse(MouseButton::Left)) = e.release_args() {
            self.pressed = false;
            if self.mouse_cursor_inside {
                self.events.push(ButtonEvent::Click);
            } else if self.was_inside {
                self.events.push(ButtonEvent::Cancel);
            }
            self.was_inside = false;
        }
    }

    /// Handle touch events.
    pub fn touch_event<E: GenericEvent, S: Into<[u32; 2]>>(&mut self,
                                                           layout: Rectangle,
                                                           transform: Matrix2d,
                                                           window_size: S,
                                                           e: &E) {
        use input::Touch;
        use math::is_inside;

        let window_size = window_size.into();
        if let Some(args) = e.touch_args() {
            let pos = args.position();
            let pos = [pos[0] * window_size[0] as f64, pos[1] * window_size[1] as f64];
            let inside = is_inside(pos, transform, layout);
            match args.touch {
                Touch::Start => {
                    if inside {
                        self.pressed = true;
                        self.was_inside = true;
                        self.mouse_cursor_inside = true;
                        self.events.push(ButtonEvent::Press);
                        self.pressed_instant = Some(Instant::now());
                    }
                }
                Touch::Move => {
                    if inside {
                        if !self.mouse_cursor_inside {
                            self.mouse_cursor_inside = true;
                            self.events.push(ButtonEvent::MouseEnter);
                        }
                    } else {
                        if self.mouse_cursor_inside {
                            self.mouse_cursor_inside = false;
                            self.events.push(ButtonEvent::MouseLeave);
                        }
                    }
                }
                Touch::End => {
                    self.pressed = false;
                    if self.mouse_cursor_inside {
                        self.events.push(ButtonEvent::Click);
                    } else if self.was_inside {
                        self.events.push(ButtonEvent::Cancel);
                    }
                    self.was_inside = false;
                    self.mouse_cursor_inside = false;
                }
                Touch::Cancel => {}
            }
        }
    }

    /// Returns `true` if keep pressed appearance for some duration to give user feedback.
    pub fn appear_pressed(&self, pressed_duration_secs: f64) -> bool {
        if let Some(ref instant) = self.pressed_instant {
            if math::duration_to_secs(&instant.elapsed()) < pressed_duration_secs {
                true
            } else {
                self.pressed
            }
        } else {
            self.pressed
        }
    }

    /// Returns the visual button state.
    pub fn state(&self, pressed_duration_secs: f64) -> ButtonState {
        ButtonVisual {
            appear_pressed: self.appear_pressed(pressed_duration_secs),
            mouse_cursor_inside: self.mouse_cursor_inside,
        }.state()
    }
}

/// Stores the current state of button.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ButtonState {
    /// Show inactive visual state.
    Inactive,
    /// Show hover visual state.
    Hover,
    /// Show press visual state.
    Press,
    /// Show cancel visual state.
    Cancel,
}

/// Stores the current visual state of button.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ButtonVisual {
    /// Whether the button appears pressed.
    pub appear_pressed: bool,
    /// Whether mouse cursor is inside button layout.
    pub mouse_cursor_inside: bool,
}

impl ButtonVisual {
    /// Gets the current button state.
    pub fn state(&self) -> ButtonState {
        use ButtonState as S;

        match (self.appear_pressed, self.mouse_cursor_inside) {
            (true, true) => S::Press,
            (true, false) => S::Cancel,
            (false, false) => S::Inactive,
            (false, true) => S::Hover,
        }
    }
}
