// Keeps track of the snake's position and direction, and provides methods to move the snake and check for collisions.

use std::vec::Vec;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub head: (i32, i32),
    pub direction: Direction,
    pub body: Vec<(i32, i32)>,
    pub grid_height: i32,
    pub grid_width: i32,
}

impl Snake {
    pub fn new(x: i32, y: i32, direction: Option<Direction>, grid_height: i32, grid_width: i32) -> Snake {
        Snake {
            head: (x, y),
            direction: direction.unwrap_or(Direction::Right),
            body: vec![(x, y)],
            grid_height,
            grid_width,
        }
    }

    pub fn move_snake(&mut self) {
        let mut new_head = self.head;
        match self.direction {
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
        }

        if self.body.contains(&new_head) {
            panic!("Snake collided with itself!");
        }
        if new_head.0 < 0 || new_head.0 >= self.grid_width || new_head.1 < 0 || new_head.1 >= self.grid_height {
            panic!("Snake collided with wall!");
        }

        // Account for snake eating in last move. 
        // If it does then head is already in body.
        if self.body[0] != self.head {
            self.body.push(self.head);
            self.body.remove(0);
        }
        self.head = new_head;
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        // Do not allow snake to move in opposite direction as it would collide with self.
        // Need to only update each game tick, or else player could change direction multiple times in one tick which would override this check.
        if matches!(new_direction, Direction::Up) && matches!(self.direction, Direction::Down) {
            return;
        }
        if matches!(new_direction, Direction::Down) && matches!(self.direction, Direction::Up) {
            return;
        }
        if matches!(new_direction, Direction::Left) && matches!(self.direction, Direction::Right) {
            return;
        }
        if matches!(new_direction, Direction::Right) && matches!(self.direction, Direction::Left) {
            return;
        }
        self.direction = new_direction;
    }

    pub fn grow_snake(&mut self) {
        self.body.push(self.head);
    }

    
}