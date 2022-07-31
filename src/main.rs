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
    let mut new_direction: u8 = RIGHT;
    let mut snake: VecDeque<(u32, u32)> = VecDeque::new();

    snake.push_back((10,10));
    snake.push_back((10,11));
    snake.push_back((10,12));
    snake.push_back((10,13));
    
    let mut time: std::time::Instant;

    loop {

    time = Instant::now();
    while time.elapsed().as_millis() < 100 {
        refresh_window(&mut window, &buffer);
        new_direction = update_direction(&window, direction, new_direction);
    };
    direction = new_direction;

    update_snake(&mut window, &mut buffer, &mut snake, &mut fruit, direction);
    
    next_frame(&mut window, &mut buffer, &snake, &fruit);
    }
}

fn update_direction (window: &Window, direction: u8, new_direction: u8) -> u8 {

    //all this was necessary to make sure the snake doesn't walk into itself when the user
    //attemps to make it walk on the opposite direction
    if window.is_key_down(Key::Up) || window.is_key_down(Key::W) || window.is_key_down(Key::K) {
        if direction == DOWN {
            return DOWN;
        }
        return UP;
    }

    if window.is_key_down(Key::Down) || window.is_key_down(Key::S) || window.is_key_down(Key::J) {
        if direction == UP {
            return UP;
        }
        return DOWN;
    }

    if window.is_key_down(Key::Right) || window.is_key_down(Key::D) || window.is_key_down(Key::L) {
        if direction == LEFT {
            return LEFT;
        }
            return RIGHT;
    }

    if window.is_key_down(Key::Left) || window.is_key_down(Key::A) || window.is_key_down(Key::H) {
        if direction == RIGHT {
            return RIGHT;
        }
            return LEFT;
    }

    new_direction
}

fn update_snake (window: &mut Window, buffer: &mut Vec<u32>, snake: &mut VecDeque<(u32, u32)>, fruit: &mut (u32, u32), direction: u8) {

    let mut is_alive = true;
    let snake_head = move_snake_head(snake, direction, &mut is_alive);

    if !is_alive {
        kill_snake(window, buffer, snake, fruit);
        snake.push_front((10, 10));
        respawn_fruit(fruit, snake);
        return;
    }
    
    snake.push_front(snake_head);

    if snake_head != *fruit {
        snake.pop_back();
    } else {
        respawn_fruit(fruit, snake);
    }
}

//move the head and return whether it survived
fn move_snake_head(snake: &mut VecDeque<(u32, u32)>, direction: u8, is_alive: &mut bool) -> (u32, u32) {
    //move the snake in the inputed direction and clamp it to the plane
    let (x, y) = *snake.front().unwrap();
    let snake_head = match direction {
        UP => if y == 0 {
            *is_alive = false;
            (x, y)
        } else {
            (x, y-1)
        }

        DOWN => if y == 29 {
            *is_alive = false;
            (x, y)
        } else {
            (x, y+1)
        }

        RIGHT => if x == 29 {
            *is_alive = false;
            (x, y) 
        } else {
            (x+1, y)
        }

        LEFT => if x == 0 {
            *is_alive = false;
            (x, y)
        } else {
            (x-1, y)
        }

        _ => (x, y)
    };
    
    //if the new snake_head collides with any of the existing snake, kill it
    for cell in snake.into_iter() {
        if snake_head == *cell {
            *is_alive = false;
            break;
        }
    }
    snake_head
}

fn kill_snake(window: &mut Window, buffer: &mut Vec<u32>, snake: &mut VecDeque<(u32, u32)>, fruit: &(u32, u32)) {
	println!("Your score: {}", snake.len());
	let mut interval = 100;
    while snake.front() != None {
        snake.pop_front();
        delay(interval);
        next_frame(window, buffer, snake, fruit);
        if interval > 10 {
			interval -= 2;
		}
    }
}

fn respawn_fruit (fruit: &mut (u32, u32), snake: &mut VecDeque<(u32, u32)>) {
    let new_fruit = (fastrand::u32(0..30), fastrand::u32(0..30));
    *fruit = new_fruit;

    //check if fruit is inside the snake
    let mut collision = false;
    for cell in snake.into_iter() {
        if new_fruit == *cell {
            collision = true;
        }
    }
    
    if collision {
        respawn_fruit(fruit, snake);
    }
}

fn next_frame(window: &mut Window, buffer: &mut Vec<u32>, snake: &VecDeque<(u32, u32)>, fruit: &(u32, u32)) {
    buffer.fill(BLACK);
    draw_plane(buffer, snake, fruit);
    refresh_window(window, buffer);
}

fn draw_plane(buffer: &mut Vec<u32>, snake: &VecDeque<(u32, u32)>, fruit: &(u32, u32)) {
    for (x, y) in snake {
        draw_square(buffer, (20*x + 20*600*y) as usize, 18, WHITE);
    }
    let (x, y) = fruit;
    draw_square(buffer, (20*x + 20*600*y) as usize, 18, RED);
}

fn draw_square(buffer: &mut Vec<u32>, corner: usize, side: usize, color: u32) {
    for i in 1..side {
        for j in 1..side {
                buffer[i*600+j+corner] = color;
        }
    }
}

fn refresh_window(window: &mut Window, buffer: &Vec<u32>) {
    window.update_with_buffer(&buffer, 600, 600)
        .expect("Couldn't refresh window.");
}

fn delay(delay: u128) {
    let time = Instant::now();
    while time.elapsed().as_millis() < delay {
        //do nothing
    };
}
