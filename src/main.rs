//temporary
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use minifb::{Window, WindowOptions};
use std::time::Instant;
use std::collections::VecDeque;

const WHITE : u32 = 16777215;
const BLACK : u32 = 0;

const UP : u8 = 1;
const DOWN : u8 = 2;
const RIGHT : u8 = 3;
const LEFT : u8 = 4;

fn main() {

    //create window
    let mut window = Window::new("Rusty Snake", 600, 600, WindowOptions::default())
        .expect("Minifb was unable to create window.");
    let mut buffer: Vec<u32> = vec![BLACK; 600*600];
    
    let mut direction: u8 = RIGHT;
    let mut snake: VecDeque<(u32, u32)> = VecDeque::new();

    snake.push_back((10,10));
    snake.push_back((10,11));
    snake.push_back((10,12));
    snake.push_back((10,13));
    
    loop {
    draw_plane(&mut buffer, &snake, WHITE, 18);
    next_frame(&mut window, &buffer);
    delay(500);
    draw_plane(&mut buffer, &snake, BLACK, 18);
    update_snake (&mut snake, direction);
    }
}

fn update_snake (snake: &mut VecDeque<(u32, u32)>, direction: u8) {
    let (x, y) = *snake.front().unwrap();
    let new_square = match direction {
        UP => (x, y+1),
        DOWN => (x, y-1),
        RIGHT => (x+1, y),
        LEFT => (x-1, y),
        _ => (x, y)
    };
    snake.push_front(new_square);
    snake.pop_back();
}

fn draw_plane(buffer: &mut Vec<u32>, snake: &VecDeque<(u32, u32)>, color: u32, side: usize) {
    for (x,y) in snake {
        draw_square(buffer, (20 * x + 20 * 600 * y) as usize, side, color);
    }
}

fn draw_square(buffer: &mut Vec<u32>, corner: usize, side: usize, color: u32) {
    for i in 1..side {
        for j in 1..side {
            buffer[i*600+j+corner] = color;
        }
    }
}

//abstraction functions
//
fn delay(delay: u128) {
    let time = Instant::now();
    while time.elapsed().as_millis() < delay {
       //do nothing 
    }; 
}

fn next_frame(window: &mut Window, buffer: &Vec<u32>) {
    window.update_with_buffer(&buffer, 600, 600)
        .expect("Minifb was unable to update the window.");
}
