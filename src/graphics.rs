/*#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]*/

use minifb::{Window, WindowOptions};
use std::collections::VecDeque;

const WHITE : u32 = 16777215;
const RED : u32 = 16711680;
const BLACK : u32 = 0;
type Cell = (usize, usize);

pub struct Graphics {
    pub window: Window,
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub square_size: usize,
}

impl Graphics {
    pub fn new(rows: usize, cols: usize, square_size: usize) -> Graphics {
        Graphics {
        window: Window::new("Rusty Snake", cols*square_size, rows*square_size, WindowOptions::default())
            .expect("Minifb couldn't create window"),
        buffer: vec![BLACK; cols*rows*square_size*square_size],
        width: cols*square_size,
        height: rows*square_size,
        square_size: square_size
        }
    }
     
    pub fn next_frame(&mut self, snake: &VecDeque<Cell>, fruit: Cell) {
        self.buffer.fill(BLACK); //erase buffer
        self.draw_plane(&snake, fruit);
        self.refresh();
    }
    
    pub fn draw_plane(&mut self, snake: &VecDeque<Cell>, fruit: Cell) {
        for (x, y) in snake {
            let corner = self.square_size * *x as usize + self.square_size * self.width * *y as usize;
            self.buffer = self.draw_square(corner, WHITE);
        }
        let (x, y) = fruit;
        let corner = self.square_size * x as usize + self.square_size * self.width * y as usize;
        self.buffer = self.draw_square(corner, RED);
    }
    
    pub fn draw_square(&self, corner: usize, color: u32) -> Vec<u32> {
        let mut buffer = self.buffer.to_vec();
        for i in 1..self.square_size-2 {
            for j in 1..self.square_size-2 {
                buffer[i * self.width + j + corner] = color;
            }
        }
        buffer
    }

    pub fn refresh(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.width, self.height)
            .expect("Couldn't refresh window.");
    }       
}
