// Import necessary modules and types from external libraries.
use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

// Import a function from your own code.
use crate::draw::draw_block;

// Define a constant color for the snake.
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

// Define an enumeration to represent directions the snake can move.
//By deriving the traits below, you get basic functionality for copying, cloning, and comparing Direction enum values without having to manually implement these traits.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Returns the opposite direction.
    pub fn opposite(&self) -> Direction {
        // Match the current direction and return the opposite direction.
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right, 
            Direction::Right => Direction::Left,
        }
    }
}

// Define a struct to represent a block with x and y coordinates.
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}


// Define a struct to represent a snake in the game.
pub struct Snake {
    direction: Direction,          // Current direction the snake is moving.
    body: LinkedList<Block>,      // Linked list to store the snake's body segments.
    tail: Option<Block>,          // Optional tail segment when the snake grows.
}


impl Snake {
    // Creates a new snake instance with an initial position.
    pub fn new(x: i32, y: i32) -> Snake {
        // Create a linked list to store the snake's body segments.
        let mut body: LinkedList<Block> = LinkedList::new();
        
        // Add three initial body segments to the snake, forming a basic shape.
        body.push_back(Block { x: x + 2, y }); // Tail segment
        body.push_back(Block { x: x + 1, y }); // Middle segment
        body.push_back(Block { x, y });         // Head segment

        // Create and return a new Snake instance with the initial direction, body, and no tail.
        Snake {
            direction: Direction::Right, // Start with the snake moving right.
            body,
            tail: None, // No tail segment initially.
        }
    }

    // Draws the snake's body on the screen.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Iterate through each block in the snake's body and draw it.
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    // Returns the position of the snake's head.
    pub fn head_position(&self) -> (i32, i32) {
        // Get a reference to the front (head) block of the snake.
        let head_block = self.body.front().unwrap();
        
        // Return the coordinates (x, y) of the head block.
        (head_block.x, head_block.y)
    }

    // Moves the snake forward in the specified direction or its current direction.
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // Set the snake's direction to the specified direction (if provided).
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        // Get the position of the snake's head.
        let (last_x, last_y): (i32, i32) = self.head_position();

         // Calculate the new block's position based on the snake's direction.
         let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1, 
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            }
        };

        // Add the new block to the front of the snake's body.
        self.body.push_front(new_block);

        // Remove the last block to maintain the snake's length.
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    // Returns the current direction of the snake's head.
    pub fn head_direction(&self) -> Direction {
        // The direction of the snake's head is stored in the 'direction' field.
        self.direction
    }

    // Calculates the next position of the snake's head based on a specified or current direction.
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        // Get the current position of the snake's head.
        let (head_x, head_y): (i32, i32) = self.head_position();

        // Initialize the moving direction with the snake's current direction.
        let mut moving_dir = self.direction;

        // Update the moving direction if a specified direction is provided.
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        // Calculate the next position based on the updated moving direction.
        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    // Restores the snake's tail by adding the last removed block back to the body.
    pub fn restore_tail(&mut self) {
        // Clone the last removed block from the tail (if it exists).
        let blk = self.tail.clone().unwrap();

        // Add the cloned block back to the end of the snake's body.
        self.body.push_back(blk);
    }

    // Checks if a specified position (x, y) overlaps with any segment of the snake's body.
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;

        // Iterate through each block in the snake's body.
        for block in &self.body {
            // Check if the specified position matches the coordinates of a block.
            if x == block.x && y == block.y {
                return true; // Overlap detected.
            }

            ch += 1;

            // Break the loop if we've checked all segments except the last one.
            if ch == self.body.len() - 1 {
                break;
            }
        }
        
        return false; // No overlap detected.
    }
}