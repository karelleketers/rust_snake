// Import necessary modules and types from external libraries.
use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

// Import modules and types from your own code.
use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

// Define constant colors for the game elements.
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

// Define constant time intervals and periods.
const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

// Define a struct to represent the game state.
pub struct Game {
    snake: Snake,         // The snake object.
    
    food_exists: bool,    // Flag indicating whether food exists.
    food_x: i32,          // X-coordinate of the food.
    food_y: i32,          // Y-coordinate of the food.

    width: i32,           // Width of the game area.
    height: i32,          // Height of the game area.

    game_over: bool,      // Flag indicating game over state.
    waiting_time: f64,    // Time elapsed since game over.
}

impl Game {
    // Constructor for creating a new game instance.
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            // Create a new snake starting at position (2, 2).
            snake: Snake::new(2, 2),

            // Initialize the waiting time (time since game over) to zero.
            waiting_time: 0.0,

            // Set the initial state of food existence to true.
            food_exists: true,

            // Set the initial position of the food (6, 4).
            food_x: 6,
            food_y: 4,

            // Store the width and height of the game area.
            width,
            height,

            // Set the initial game over state to false.
            game_over: false,
        }
    }

    // Handles key presses to control the game.
    pub fn key_pressed(&mut self, key: Key) {
        // If the game is already over, ignore key presses.
        if self.game_over {
            return;
        }

        // Determine the direction based on the pressed key.
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None, // Ignore other keys.
        };

        // Check if the selected direction is the opposite of the current snake direction.
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return; // Ignore opposite direction to prevent self-collisions.
        }

        // Update the snake's state based on the chosen direction.
        self.update_snake(dir);
    }

    // Draws the game elements on the screen.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Draw the snake on the screen.
        self.snake.draw(con, g);

        // If food exists, draw it on the screen.
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // Draw the game borders to create a game area.
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g); // Top border
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g); // Bottom border
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g); // Left border
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g); // Right border

        // If the game is over, draw a semi-transparent game over screen.
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    // Updates the game state based on elapsed time.
    pub fn update(&mut self, delta_time: f64) {
        // Increment the waiting time by the elapsed delta time.
        self.waiting_time += delta_time;

        // If the game is over, check if it's time to restart.
        if self.game_over {
            // Check if enough time has passed to restart the game.
            if self.waiting_time > RESTART_TIME {
                self.restart(); // Restart the game.
            }
            return; // Return early if the game is over.
        }

        // If food doesn't exist, add a new food item.
        if !self.food_exists {
            self.add_food();
        }

        // Check if enough time has passed to update the snake's movement.
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None); // Update the snake's movement.
        }
    }

    // Checks if the snake has eaten the food and updates the game state.
    fn check_eating(&mut self) {
        // Get the current position of the snake's head.
        let (head_x, head_y): (i32, i32) = self.snake.head_position();

        // Check if food exists and if the snake's head is at the same position as the food.
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            // Food is eaten, so mark it as not existing and restore the snake's tail.
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    // Checks if the snake is alive and within the game boundaries.
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        // Calculate the next head position based on the given direction.
        let (next_x, next_y) = self.snake.next_head(dir);

        // Check if the snake overlaps with its own tail.
        if self.snake.overlap_tail(next_x, next_y) {
            return false; // Snake is not alive if it overlaps with its tail.
        }

        // Check if the next position is within the game boundaries.
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    // Adds a new food item to the game.
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        // Generate random coordinates for the new food item.
        let mut new_x = rng.gen_range(1..=self.width);
        let mut new_y = rng.gen_range(1..=self.height);

        // Ensure that the new food item doesn't overlap with the snake's tail.
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..=self.width);
            new_y = rng.gen_range(1..=self.height);
        }

        // Set the position and existence of the new food item.
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    // Updates the snake's movement and game state.
    fn update_snake(&mut self, dir: Option<Direction>) {
        // Check if the snake is alive based on the given direction.
        if self.check_if_snake_alive(dir) {
            // Move the snake forward in the given direction.
            self.snake.move_forward(dir);
            
            // Check if the snake has eaten the food.
            self.check_eating();
        } else {
            // If the snake is not alive, mark the game as over.
            self.game_over = true;
        }
        
        // Reset the waiting time (time since last update).
        self.waiting_time = 0.0;
    }

    // Restarts the game by resetting its state.
    fn restart(&mut self) {
        // Create a new snake starting at position (2, 2).
        self.snake = Snake::new(2, 2);
        
        // Reset the waiting time (time since last update).
        self.waiting_time = 0.0;
        
        // Set the initial state of food existence to true.
        self.food_exists = true;
        
        // Set the initial position of the food (6, 4).
        self.food_x = 6;
        self.food_y = 4;
        
        // Set the initial game over state to false.
        self.game_over = false;
    }
}