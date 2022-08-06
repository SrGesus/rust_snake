#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use minifb::Key;
use std::time::Instant;
use std::collections::VecDeque;
use rust_snake::graphics::*;

fn main() {
    let (rows, cols, square_size) = (10, 10, 50);
    let mut game = Game::new(rows, cols, square_size);
    let bfs = Bfs::new(rows, cols);
    
    game.next_frame();

    loop {
        game.update_direction();
        game.update_snake();
        game.next_frame();
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction { Up, Down, Right, Left }

impl Direction {
    pub fn opposite(&self) -> Direction {
        use crate::Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right,
        }
    }
}

pub type Cell = (u32, u32);

struct Game {
    graphics: Graphics,
    cols: u32,
    rows: u32,
    snake: VecDeque<Cell>,
    direction: Direction,
    fruit: Cell,
    delay: u128
}

impl Game {
    pub fn new(rows: usize, cols: usize, square_size: usize) -> Game {
        let mut game = Game {
            graphics: Graphics::new(rows, cols, square_size),
            cols: cols as u32,
            rows: rows as u32,
            snake: VecDeque::new(),
            direction: Direction::Right,
            fruit: (fastrand::u32(1..cols as u32), fastrand::u32(1..rows as u32)),
            delay: 100
        };
        game.snake.push_back((0, 0));

        game
    }
    
    pub fn update_direction(&mut self) {
        use crate::Direction::*;

        let mut new_direction = self.direction;

        let time = Instant::now();
        while time.elapsed().as_millis() < self.delay {
            self.graphics.refresh();

            self.graphics.window.get_keys().iter().for_each(|key|
                match key {
                    Key::Up | Key::W => new_direction = Up,
                    Key::Down | Key::S => new_direction = Down,
                    Key::Right | Key::D => new_direction = Right,
                    Key::Left | Key::A => new_direction = Left,
                    _ => ()
                }
            );
        }

        if new_direction != self.direction.opposite() {
            self.direction = new_direction;
        }
    }

    pub fn update_snake(&mut self) {
        let mut is_alive = true;
        let snake_head = self.get_snake_head(&mut is_alive);

        if !is_alive {
            self.kill_snake();
            self.respawn_fruit();
            return;
        }

        self.snake.push_front(snake_head);

        if snake_head != self.fruit {
            self.snake.pop_back();
        } else {
            self.respawn_fruit();
        }
    }

    //return the new snake head
    fn get_snake_head(&mut self, is_alive: &mut bool) -> (u32, u32) {
        //move the snake head in the inputed direction and clamp it to the plane
        let (x, y) = *self.snake.front().unwrap();
        
        use crate::Direction::*;
        let snake_head = match self.direction {

            Up => if y == 0 {
                *is_alive = false;
                (x, y)
            } else {
                (x, y-1)
            }

            Down => if y == self.rows-1 {
                *is_alive = false;
                (x, y)
            } else {
                (x, y+1)
            }

            Right => if x == self.cols-1 {
                *is_alive = false;
                (x, y) 
            } else {
                (x+1, y)
            }

            Left => if x == 0 {
                *is_alive = false;
                (x, y)
            } else {
                (x-1, y)
            }
        };

        //if the new snake_head collides with any of the existing snake, kill it
        for cell in &self.snake {
            if snake_head == *cell {
                *is_alive = false;
                break;
            }
        }
        snake_head
    }
    
    fn kill_snake(&mut self) {
        println!("Your score was: {}", self.snake.len());
        let mut delay = 100;
        while self.snake.front() != None {
            self.snake.pop_front();
            sleep(delay);
            self.next_frame();
            if delay > 10 {
                delay -= 4;
            }
        }
        self.snake.push_front((0, 0));
        self.direction = Direction::Right;
    }

    fn respawn_fruit(&mut self) {
        let new_fruit = (fastrand::u32(0..self.cols), fastrand::u32(0..self.rows));
        self.fruit = new_fruit;

        //make sure fruit doesn't spawn inside the snake
        let mut collision = false;
        for cell in &self.snake {
            if new_fruit == *cell {
                collision = true;
            }
        }

        if collision {
            self.respawn_fruit();
        }
    }
	pub fn next_frame(&mut self) {
		self.graphics.next_frame(&self.snake, self.fruit);
	}
     
}


struct Bfs {
    grid: Vec<Vec<bool>>,
    visited: Vec<Vec<bool>>,
    distance: Vec<Vec<u32>>,
    source: Vec<Vec<(u32, u32)>>,
}

impl Bfs {
    pub fn new(rows: usize, cols: usize) -> Bfs {
        Bfs {
            grid: vec![vec![false; rows]; cols],
            visited: vec![vec![false; rows]; cols],
            distance: vec![vec![u32::MAX; rows]; cols],
            source: vec![vec![(0, 0); rows]; cols],
        }
    }

    pub fn update(&mut self, game: &Game) {
        for (x, y) in &game.snake {
            self.grid[*x as usize][*y as usize] = true;
        }
    }
}


fn sleep(delay: u128) {
    let time = Instant::now();
    while time.elapsed().as_millis() < delay {
        //do nothing
    }
}
