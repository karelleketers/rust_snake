// Import external crates and modules.
extern crate rand;
extern crate piston_window;

// Import custom modules.
mod draw;
mod snake;
mod game;

// Import necessary types and functions from external libraries.
use piston_window::*;
use piston_window::types::Color;

// Import the 'Game' struct from the 'game' module.
use game::Game;

// Define a constant background color.
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    // Define the dimensions of the game window.
    let (width, height) = (20, 20);

    // Create a PistonWindow with specified settings.
    let mut window: PistonWindow = WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new instance of the 'Game' struct.
    let mut game = Game::new(width, height);

    // Main game loop.
    while let Some(event) = window.next() {
        // Handle keyboard input events.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // Draw the game on the window.
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);  // Clear the window with the background color.
            game.draw(&c, g);     // Draw the game elements.
        });

        // Update the game state.
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
