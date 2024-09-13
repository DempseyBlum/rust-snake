pub mod snake;

extern crate piston_window;
use color::{BLACK, RED, WHITE};
use piston_window::*;
use piston_window::grid::Grid;


pub struct App {
    grid: Grid,
    snake: snake::Snake,
    score: u32,
    current_direction: snake::Direction,
}

impl App {
    pub fn new(columns: u32, rows: u32) -> App {
        let grid = Grid {cols: 50, rows: 50, units: 30.0};
        let mut snake = snake::Snake::new(0, 0, Some(snake::Direction::Right), grid.cols as i32, grid.rows as i32);

        App {
            grid,
            snake,
            score: 0,
            current_direction: snake::Direction::Right,
        }
    }

    pub fn get_snake(&self) -> &snake::Snake {
        &self.snake
    }

    pub fn change_direction(&mut self, new_direction: snake::Direction) {
        self.current_direction = new_direction;
    }

    pub fn update (&mut self) {
        // Only update direction on each update, or else player can get around the moving backwards collision check.
        self.snake.change_direction(self.current_direction);
        // TODO: Check if snake has eaten food
        self.snake.move_snake();
    }

    // Temporary function for testing
    pub fn eat_food(&mut self) {
        self.snake.grow_snake();
    }
}

fn main() {
    let mut app = App::new(30, 30);

    let mut window: PistonWindow = WindowSettings::new("Snake Game!", (app.grid.rows as f64 *app.grid.units, app.grid.cols as f64 *app.grid.units))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let event_settings = EventSettings::new().ups(10);
    let mut events = Events::new(event_settings);
    let mut already_pressed = true;

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            let mut snake_blocks = Vec::new();
            let snake = app.get_snake();

            // Add body to snake blocks
            for block in snake.body.iter() {
                let square = rectangle::square(block.0 as f64 * app.grid.units, block.1 as f64 * app.grid.units, 30.0);
                snake_blocks.push(square);
            }
            // Add head to snake blocks
            snake_blocks.push(rectangle::square(snake.head.0 as f64 * app.grid.units, snake.head.1 as f64 * app.grid.units, 30.0));
    
            window.draw_2d(&e, |c, g, _| {
                clear(BLACK, g);
    
                for cell in app.grid.cells() {
                    let square = rectangle::square(app.grid.x_pos(cell) as f64, app.grid.y_pos(cell) as f64, app.grid.units-1.0);
                    rectangle(WHITE, square, c.transform, g);
                }
    
                for block in snake_blocks {
                    rectangle(RED, block, c.transform, g);
                }
            });
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if !already_pressed {
                match key {
                    Key::Up => app.change_direction(snake::Direction::Up),
                    Key::Down => app.change_direction(snake::Direction::Down),
                    Key::Left => app.change_direction(snake::Direction::Left),
                    Key::Right => app.change_direction(snake::Direction::Right),
                    // Temporary for testing
                    Key::Space => app.eat_food(),
                    _ => {}
                }
            }
        }

        if let Some(_) = e.update_args() {
            app.update();
            already_pressed = false;
        }


    }
}