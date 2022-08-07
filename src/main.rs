#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use minifb::Key;
use std::time::Instant;
use std::collections::VecDeque;
use rust_snake::graphics::*;

fn main() {
    let (rows, cols, square_size) = (10, 10, 30);
    let mut game = Game::new(rows, cols, square_size);
    let mut bfs = Bfs::new(rows, cols);
    
    game.next_frame();

    loop {
        game.update_direction();
        game.update_snake();
        bfs.update(&game);
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

    pub fn get_direction(source: Cell, destination: Cell) -> Direction {
        let (x, y) = source;
        let (nx, ny) = destination;
        
        let (x, y) = ((nx-x) as i8, (ny-y) as i8);
        
        use crate::Direction::*;

        match (x, y) {
            (0, 1) => Up,
            (0, -1) => Down,
            (1, 0) => Right,
            (-1, 0) => Left,
            _ => Up 
        }
    }
    
    pub fn get_vector(self) -> (i32, i32) {
        use crate::Direction::*;
        match self {
            Up => (0, 1),
            Down => (0, -1),
            Right => (1, 0),
            Left => (-1, 0)
        }
    }

    pub fn iter() -> [Direction; 4] {
        use crate::Direction::*;
        [Up, Down, Right, Left]
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
    rows: usize,
    cols: usize,
    grid: Vec<Vec<bool>>,
    source: Vec<Vec<Cell>>,
    cost: Vec<Vec<u32>>,
    visited: Vec<Vec<bool>>,
    //source, destination, cost
    queue: VecDeque<(Cell, Cell, u32)>,
    path: VecDeque<Cell>
}

impl Bfs {
    pub fn new(rows: usize, cols: usize) -> Bfs {
        Bfs {
            rows: rows,
            cols: cols,
            grid: vec![vec![false; rows]; cols],
            source: vec![vec![(0, 0); rows]; cols],
            cost: vec![vec![u32::MAX; rows]; cols],
            visited: vec![vec![false; rows]; cols],
            queue: VecDeque::new(),
            path: VecDeque::new()
        }
    }

    pub fn update(&mut self, game: &Game) {
        self.grid = vec![vec![false; self.rows]; self.cols]; //reset the grid
        for (x, y) in &game.snake {
            self.grid[*x as usize][*y as usize] = true;
        }
        for x in 0..self.cols {
            println!(" ");
            for y in 0..self.rows {
                print!("{} ", self.grid[x][y] as u8);
            }
        }
        println!("\n\n");
        
        while self.queue.front() != None {

        }
        
    }
    
    //returns whether the new cell is in bounds
    fn in_bounds(&mut self, source: Cell, direction: Direction) -> bool {
        use crate::Direction::*;
        let (x, y) = source;

        match direction {
            Up => if y == 0 {
                false
            } else {
                true
            },
            Down => if y == (self.rows-1) as u32 {
                false
            } else {
                true
            },
            Right => if x == (self.cols-1) as u32 {
                false
            } else {
                true
            },
            Left => if x == 0 {
                false
            } else {
                true
            }
        }
    }

    fn add_edges(&mut self, source: Cell, cost: u32) {
        let (x, y) = source;
        
        for direction in Direction::iter() {
            if self.in_bounds(source, direction) {
                let (dx, dy) = direction.get_vector();
                let nx = (x as i32 + dx) as u32;
                let ny = (y as i32 + dy) as u32;

                if !self.visited[nx as usize][ny as usize] {
                    self.queue.push_back((source, (nx, ny), cost+1));
                }
            }
        }


        /*
        if x > 0 {
            self.queue.push_back((source, (x-1, y), cost + 1));
        }

        if x < (self.cols-1) as u32 {
            self.queue.push_back((source, (x+1, y), cost + 1));
        }

        if y > 0 {
            self.queue.push_back((source, (x, y-1), cost + 1));
        }
        
        if y < (self.rows-1) as u32 {
            self.queue.push_back((source, (x, y+1), cost + 1));
        }*/
    }
}


fn sleep(delay: u128) {
    let time = Instant::now();
    while time.elapsed().as_millis() < delay {
        //do nothing
    }
}
