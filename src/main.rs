#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use minifb::Key;
use std::time::Instant;
use std::collections::VecDeque;
use rust_snake::graphics::*;

fn main() {
    let (rows, cols, square_size) = (20, 20, 20);
    let mut game = Game::new(rows, cols, square_size);
    let mut bfs = Bfs::new(rows, cols);
    
    game.next_frame();
    bfs.update(&game);
    
    loop {
        bfs.update_direction(&mut game);
//        game.update_direction();
        //sleep(1);
        game.update_snake(&mut bfs);
        game.next_frame();
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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
        
        let (x, y) = (nx as i8 - x as i8, ny as i8 - y as i8);
        
        use crate::Direction::*;
        
        if y > 1 {
            return Up
        }
        if y < -1 {
            return Down
        }
        if x > 1 {
            return Left
        }
        if y < -1 {
            return Right
        }
        
        match (x, y) {
            (0, -1) => Up,
            (0, 1) => Down,
            (1, 0) => Right,
            (-1, 0) => Left,
            _ => Right
        }
    }
    
    pub fn get_vector(self) -> (i32, i32) {
        use crate::Direction::*;
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Right => (1, 0),
            Left => (-1, 0)
        }
    }

    pub fn iter() -> [Direction; 4] {
        use crate::Direction::*;
        [Up, Down, Right, Left]
    }
}

pub type Cell = (usize, usize);

struct Game {
    graphics: Graphics,
    cols: usize,
    rows: usize,
    snake: VecDeque<Cell>,
    direction: Direction,
    fruit: Cell,
    delay: u128
}

impl Game {
    pub fn new(rows: usize, cols: usize, square_size: usize) -> Game {
        let mut game = Game {
            graphics: Graphics::new(rows, cols, square_size),
            cols: cols,
            rows: rows,
            snake: VecDeque::new(),
            direction: Direction::Right,
            fruit: (fastrand::usize(2..cols-1), fastrand::usize(2..rows-1)),
            delay: 50
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

    pub fn update_snake(&mut self, bfs: &mut Bfs) {
        let mut is_alive = true;
        let snake_head = self.get_snake_head(&mut is_alive);

        if !is_alive {
            self.kill_snake();
            self.respawn_fruit();
            bfs.update(self);
            return;
        }

        self.snake.push_front(snake_head);

        if snake_head != self.fruit {
            self.snake.pop_back();
        } else {
            self.respawn_fruit();
            bfs.update(self);
        }
    }

    //return the new snake head
    fn get_snake_head(&mut self, is_alive: &mut bool) -> (usize, usize) {
        //move the snake head in the inputed direction and clamp it to the plane
        let (x, y) = *self.snake.front().unwrap();
        
        use crate::Direction::*;
        let snake_head = match self.direction {

            Up => if y == 0 {
                (x, self.rows-1)
            } else {
                (x, y-1)
            }

            Down => if y == self.rows-1 {
                (x, 0)
            } else {
                (x, y+1)
            }

            Right => if x == self.cols-1 {
                (0, y) 
            } else {
                (x+1, y)
            }

            Left => if x == 0 {
                (self.cols-1, y)
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
        let mut delay = self.delay;
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
        let new_fruit = (fastrand::usize(0..self.cols), fastrand::usize(0..self.rows));
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
    visited: Vec<Vec<bool>>,
    //source, cost
    queue: VecDeque<Cell>,
    path: VecDeque<Cell>
}

impl Bfs {
    pub fn new(rows: usize, cols: usize) -> Bfs {
        Bfs {
            rows: rows,
            cols: cols,
            grid: vec![vec![false; rows]; cols],
            source: vec![vec![(0, 0); rows]; cols],
            visited: vec![vec![false; rows]; cols],
            queue: VecDeque::new(),
            path: VecDeque::new()
        }
    }

    pub fn update(&mut self, game: &Game) {
        let (rows, cols) = (self.rows, self.cols);
        *self = Bfs::new(rows, cols);
        for (x, y) in &game.snake {
            self.grid[*x][*y] = true;
        }
        
        let (x, y) = *game.snake.front().unwrap();
        self.queue.push_back((x, y));
        let snake_head = (x, y);
        self.source[x][y] = (x, y);
        let (x, y) = game.fruit;
        let (x, y) = (x, y);

        while self.queue.front() != None {
            //add the adjacent squares 
            let source = self.queue.pop_front().unwrap();
            self.add_edges(source);

            //if the fruit has been reached
            if self.visited[x][y] == true {
                self.get_path(snake_head, game.fruit);
                break;
            }
        }
        
    }

    fn get_path(&mut self, snake_head: Cell, fruit: Cell) {
        self.path = VecDeque::new();
        self.path.push_front(fruit);
        let (x, y) = fruit;
        let (x, y) = (x, y);
        let mut cell = self.source[x][y];

        while cell != snake_head {
            let (x, y) = cell;
            let (cx, cy) = cell;
            let (hx, hy) = snake_head;
            self.path.push_front(cell);
            cell = self.source[x][y];
        }
    }

    pub fn update_direction(&mut self, game: &mut Game) {
        if self.path.is_empty() {
            game.kill_snake();
            game.respawn_fruit();
            self.update(game);
            self.update_direction(game);
            return;
    
        }
        let (x, y) = *game.snake.front().unwrap();
        let (nx, ny) = self.path.pop_front().unwrap();
        game.direction = Direction::get_direction((x, y), (nx, ny));
    }

    fn add_edges(&mut self, source: Cell) {
        let (x, y) = source;
        
        for direction in Direction::iter() {
//            if self.in_bounds(source, direction) {
//                let (dx, dy) = direction.get_vector();
//                let nx = (x as i32 + dx) as usize;
//                let ny = (y as i32 + dy) as usize;
                let (nx, ny) = self.get_next_cell(source, direction);
                if !self.visited[nx][ny] && !self.grid[nx][ny]{
                    self.queue.push_back((nx, ny));
                    self.source[nx][ny] = source;
                    self.visited[nx][ny] = true;
                }
        }
    }

    //returns whether the new cell is in bounds
    fn get_next_cell(&mut self, source: Cell, direction: Direction) -> (usize, usize) {
        use crate::Direction::*;
        let (x, y) = source;

        match direction {
            Up => if y == 0 {
                (x, self.rows-1)
            } else {
                (x, y-1)
            },
            Down => if y == self.rows-1 {
                (x, 0)
            } else {
                (x, y+1)
            },
            Right => if x == (self.cols-1) {
                (0, y)
            } else {
                (x+1, y)
            },
            Left => if x == 0 {
                (self.cols-1, y)
            } else {
                (x-1, y)
            }
        }
    }
}


fn sleep(delay: u128) {
    let time = Instant::now();
    while time.elapsed().as_millis() < delay {
        //do nothing
    }
}
