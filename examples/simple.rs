extern crate leechbar;
use leechbar::{Alignment, Background, BarBuilder, Component, ComponentPosition, Text};
use std::time::Duration;

struct MyComponent;

impl Component for MyComponent {
    // No background image
    fn background(&mut self) -> Option<Background> {
        None
    }

    // Print "Hello, World!" as text
    fn text(&mut self) -> Option<Text> {
        Some(Text::new(String::from("Hello, World")))
    }

    // First element on the left side
    fn position(&mut self) -> ComponentPosition {
        ComponentPosition::new(Alignment::CENTER, 0)
    }

    // Do this only once
    fn timeout(&mut self) -> Option<Duration> {
        None
    }

    // Ignore all events
    fn event(&mut self) {}
}

fn main() {
    // Create a new bar
    let mut bar = BarBuilder::new().spawn().unwrap();
    // Add an instance of your component to your bar
    bar.add(MyComponent {});
    // Start the event loop that handles all X events
    bar.start_event_loop();
}
