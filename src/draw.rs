// Import necessary modules from the piston_window crate.
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

// Define a constant for the block size. This determines the size of game elements.
const BLOCK_SIZE: f64 = 25.0;

// Convert a game coordinate (integer) to a graphical coordinate (floating-point).
pub fn to_coord(game_coord: i32) -> f64 {
    // Multiply the game coordinate by the BLOCK_SIZE to get the graphical coordinate.
    (game_coord as f64) * BLOCK_SIZE
}

// Convert a game coordinate (integer) to a graphical coordinate (unsigned 32-bit integer).
pub fn to_coord_u32(game_coord: i32) -> u32 {
    // Use the 'to_coord' function to get the graphical coordinate and cast it as u32.
    to_coord(game_coord) as u32
}

// Function to draw a colored block at a specified position.
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    // Convert game coordinates (x, y) to graphical coordinates (gui_x, gui_y).
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    // Draw a rectangle with the specified color and size at (gui_x, gui_y).
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

// Function to draw a colored rectangle at a specified position with width and height.
pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    // Convert game coordinates (x, y) to graphical coordinates (x, y).
    let x = to_coord(x);
    let y = to_coord(y);

    // Draw a rectangle with the specified color, size, and position.
    rectangle(
        color,
        [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
        con.transform,
        g,
    );
}
