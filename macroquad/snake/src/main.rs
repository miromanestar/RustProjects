use macroquad::prelude::*;

const SQUARES: i16 = 16;

type Point = (i16, i16);

struct Snake {
    head: (i16, i16),
    body: Vec<(i16, i16)>,
    dir: Point
}

#[macroquad::main("Snake")]
async fn main() {

    let mut snake = Snake {
        head: (0, 0),
        dir: (1, 0),
        body: Vec::new()
    };

    let mut last_update = get_time();
    let mut speed = 0.25;
    let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));

    loop {
        clear_background(Color::from_hex(0x2E3440));

        if (is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D)) && snake.dir != (-1, 0) {
            snake.dir = (1, 0);
        } else if (is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A)) && snake.dir != (1, 0) {
            snake.dir = (-1, 0);
        } else if (is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W)) && snake.dir != (0, 1) {
            snake.dir = (0, -1);
        } else if (is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S)) && snake.dir != (0, -1) {
            snake.dir = (0, 1);
        }

        if get_time() - last_update > speed {
            last_update = get_time();

            let (x, y) = snake.head;
            let (dx, dy) = snake.dir;

            // Move the body
            for i in (0..snake.body.len()).rev() {
                if i == 0 {
                    snake.body[i] = snake.head;
                } else {
                    snake.body[i] = snake.body[i - 1];
                }
            }

            snake.head = (x + dx, y + dy);

            // Check for collision with the body
            for (x, y) in &snake.body {
                if *x == snake.head.0 && *y == snake.head.1 {
                    snake = Snake {
                        head: (0, 0),
                        dir: (1, 0),
                        body: Vec::new()
                    };
                    speed = 0.5;
                    fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                    break;
                }
            }
        }

        // Check for collision with the fruit
        if snake.head == fruit {
            fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
            snake.body.push(snake.head);
            speed *= 0.95;
        }

        // Check for collision with the walls
        if snake.head.0 < 0 || snake.head.0 >= SQUARES || snake.head.1 < 0 || snake.head.1 >= SQUARES {
            snake = Snake {
                head: (0, 0),
                dir: (1, 0),
                body: Vec::new()
            };
            speed = 0.5;
            fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
        }

        // Draw the damn things
        draw_grid();
        draw_fruit(&fruit);
        draw_player(&snake);

        next_frame().await
    }
}

fn draw_grid() {
    let square_size = screen_width() / SQUARES as f32;
    let color = Color::from_hex(0x3B4252);
    for i in 0..SQUARES {
        let c = i as f32;
        
        draw_line(c * square_size, 0.0, c * square_size, screen_height(), 1.0, color);
        draw_line(0.0, c * square_size, screen_width(), c * square_size, 1.0, color);
    }
}

fn draw_player(snake: &Snake) {
    let square_size = screen_width() / SQUARES as f32;
    let headColor = Color::from_hex(0xD8DEE9);
    let bodyColor = Color::from_hex(0xECEFF4);

    let (x, y) = snake.head;
    draw_rectangle(x as f32 * square_size, y as f32 * square_size, square_size, square_size, headColor);
    
    for (x, y) in &snake.body {
        draw_rectangle(*x as f32 * square_size, *y as f32 * square_size, square_size, square_size, bodyColor);
    }
}

fn draw_fruit(fruit: &Point) {
    let square_size = screen_width() / SQUARES as f32;
    let color = Color::from_hex(0xA3BE8C);
    let (x, y) = fruit;

    draw_rectangle(*x as f32 * square_size, *y as f32 * square_size, square_size, square_size, color);
}