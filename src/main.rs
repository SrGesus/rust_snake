//temporary
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use minifb::{Window, WindowOptions, Key};
use std::time::Instant;
use std::collections::VecDeque;

const WHITE : u32 = 16777215;
const RED : u32 = 16711680;
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
    
    let mut fruit: (u32, u32) = (20,20);
    let mut direction: u8 = RIGHT;
    let mut snake: VecDeque<(u32, u32)> = VecDeque::new();

    snake.push_back((10,10));
    snake.push_back((10,11));
    snake.push_back((10,12));
    snake.push_back((10,13));
    
    let mut time: std::time::Instant;

    loop {
    draw_plane(&mut buffer, &snake, &fruit, false, 18);

    time = Instant::now();
    while time.elapsed().as_millis() < 200 {
        next_frame(&mut window, &buffer);
        direction = update_direction(&window, direction);
    };

    //erase board
    draw_plane(&mut buffer, &snake, &fruit, true, 18);
    
    update_snake(&mut snake, &mut fruit, direction);
    }
}

fn update_direction (window: &Window,direction: u8) -> u8 {
    if window.is_key_down(Key::Up) {return UP;}
    if window.is_key_down(Key::Down) {return DOWN;}
    if window.is_key_down(Key::Right) {return RIGHT;}
    if window.is_key_down(Key::Left) {return LEFT;}
    direction
}

fn update_snake (snake: &mut VecDeque<(u32, u32)>, fruit: &mut (u32, u32), direction: u8) {
    let (x, y) = *snake.front().unwrap();
    let mut is_alive = true;

    //move the snake in the inputed direction and clamp it to the plane
    let snake_head = match direction {
        UP => if y == 0 {
            is_alive = false;
            (x, y)
        } else {
            (x, y-1)
        }

        DOWN => if y == 29 {
            is_alive = false;
            (x, y)
        } else {
            (x, y+1)
        }

        RIGHT => if x == 29 {
            is_alive = false;
            (x, y) 
        } else {
            (x+1, y)
        }

        LEFT => if x == 0 {
            is_alive = false;
            (x, y)
        } else {
            (x-1, y)
        }

        _ => (x, y)
    };
    
    //if the new snake_head collides with any of the existing snake, kill it
    for square in snake.into_iter() {
        if snake_head == *square {
            is_alive = false;
            break;
        }
    }

    if !is_alive {
        snake.clear();
        snake.push_front((10, 10));
        respawn_fruit(fruit);
        return;
    }

    snake.push_front(snake_head);

    if snake_head != *fruit {
        snake.pop_back();
    } else {
        respawn_fruit(fruit);
    }
}

fn respawn_fruit (fruit: &mut (u32, u32)) {
    let mut new_fruit = (fastrand::u32(0..30), fastrand::u32(0..30));
    *fruit = new_fruit;
}

fn draw_fruit (buffer: &mut Vec<u32>, fruit: &(u32, u32)) {
    let (x, y) = *fruit;
    draw_square(buffer, (20 * x + 20 * 600 * y) as usize, 18, RED);
}

fn draw_plane(buffer: &mut Vec<u32>, snake: &VecDeque<(u32, u32)>, fruit: &(u32, u32), do_erase: bool, side: usize) {
    let mut snake_color = WHITE;
    let mut fruit_color = RED;
    if do_erase {
        snake_color = BLACK;
        fruit_color = BLACK;
    }
    for (x,y) in snake {
        draw_square(buffer, (20 * x + 20 * 600 * y) as usize, side, snake_color);
    }
    let (x,y) = fruit;
    draw_square(buffer, (20 * x + 20 * 600 * y) as usize, 18, fruit_color);
}

fn draw_square(buffer: &mut Vec<u32>, corner: usize, side: usize, color: u32) {
    for i in 1..side {
        for j in 1..side {
            buffer[i*600+j+corner] = color;
        }
    }
}

fn next_frame(window: &mut Window, buffer: &Vec<u32>) {
    window.update_with_buffer(&buffer, 600, 600)
        .expect("Minifb was unable to update the window.");
}
