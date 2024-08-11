use std::collections::LinkedList;

use piston_window::{ellipse, rectangle, Context, G2d, Key};
use rand::Rng;

pub(crate) const SCREEN_HEIGHT: i32 = 24;
pub(crate) const SCREEN_WIDTH: i32 = 32;

const MOVING_TIME: f64 = 0.1;

pub(crate) const CORDINATE_INCREMENTOR: f64 = 25.0f64;

pub fn to_coordinate(incr: i32) -> f64 {
    incr as f64 * CORDINATE_INCREMENTOR
}

#[derive(Debug, Clone, PartialEq)]
enum SnakeDirection {
    Up,
    Down,
    Right,
    Left,
}
#[derive(Debug, Clone, Copy)]
struct SnakePosition {
    x: f64,
    y: f64,
}

impl SnakePosition {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

pub struct GamePosition {
    snake_body: LinkedList<SnakePosition>,

    food_x: f64,
    food_y: f64,

    direction: SnakeDirection,

    waitime_time: f64,

    should_move: bool,
    game_over: bool,
}

impl GamePosition {
    pub(crate) fn new(rectangle_x: f64, rectangle_y: f64, food_x: f64, food_y: f64) -> Self {
        println!("{} {} {} {}", rectangle_x, rectangle_y, food_x, food_y);
        let direction = SnakeDirection::Right;
        let mut snake_body: LinkedList<SnakePosition> = LinkedList::new();

        snake_body.push_back(SnakePosition {
            x: to_coordinate(2),
            y: to_coordinate(0),
        });

        snake_body.push_back(SnakePosition {
            x: to_coordinate(1),
            y: to_coordinate(0),
        });
        snake_body.push_back(SnakePosition {
            x: to_coordinate(0),
            y: to_coordinate(0),
        });
        Self {
            snake_body,
            food_x,
            food_y,
            direction,
            waitime_time: 0.0,
            should_move: false,
            game_over: false,
        }
    }

    pub(crate) fn check_key_event(&mut self, key: Key) {
        println!("key pressed - start {:?}", self.direction);
        match key {
            Key::Up => {
                if self.direction != SnakeDirection::Down {
                    self.direction = SnakeDirection::Up
                }
            }
            Key::Down => {
                if self.direction != SnakeDirection::Up {
                    self.direction = SnakeDirection::Down
                }
            }
            Key::Left => {
                if self.direction != SnakeDirection::Right {
                    self.direction = SnakeDirection::Left
                }
            }
            Key::Right => {
                if self.direction != SnakeDirection::Left {
                    self.direction = SnakeDirection::Right
                }
            }
            _ => (),
        };
        println!("key pressed - end {:?}", self.direction);
    }

    pub fn draw(&mut self, c: &Context, g: &mut G2d) {
        if self.game_over {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], g);
        } else {
            piston_window::clear([1.0, 1.0, 1.0, 1.0], g);
            self.draw_snake(*c, g);
            self.draw_food(*c, g);
        }
    }

    fn draw_snake(&mut self, c: Context, g: &mut G2d) {
        let transform = c.transform;

        let mut is_fist = true;
        for snk in &self.snake_body {
            // println!("{} {} ", snk.x, snk.y);
            let color = if is_fist {
                [0.0, 1.0, 0.0, 1.8]
            } else {
                [0.0, 1.0, 0.0, 0.8]
            };
            ellipse(
                color,
                [snk.x, snk.y, CORDINATE_INCREMENTOR, CORDINATE_INCREMENTOR],
                transform,
                g,
            );
            if is_fist {
                is_fist = false;
            }
        }
        // println!("snake body length {}", self.snake_body.len());
    }

    fn draw_food(&mut self, c: Context, g: &mut G2d) {
        rectangle(
            [1.0, 0.0, 0.0, 1.0],
            [
                self.food_x,
                self.food_y,
                CORDINATE_INCREMENTOR,
                CORDINATE_INCREMENTOR,
            ],
            c.transform,
            g,
        );
    }

    pub(crate) fn update_time(&mut self, dt: f64) {
        self.waitime_time += dt;

        if self.waitime_time > MOVING_TIME {
            self.should_move = true;
            self.waitime_time = 0.0;
            if self.game_over {
                self.game_over = false;
            }
        }

        if self.should_move {
            self.should_move = false;
            if self.has_snake_touched_itself() {
                self.restart_game();
                return;
            }
            self.let_wall_cross();
            self.move_forward();
            if self.snake_ate_food() {
                self.generate_food();
                self.add_snake_length();
            }
        }
    }

    fn snake_ate_food(&self) -> bool {
        let snake = self.snake_body.front();

        match snake {
            Some(s) => {
                if s.x == self.food_x && s.y == self.food_y {
                    return true;
                }
            }
            None => (),
        }
        false
    }

    fn generate_food(&mut self) {
        println!("generate_food");
        let mut rand_rang = rand::thread_rng();
        let mut x = 0.0;
        let mut y = 0.0;
        let mut snk_all = true;

        while snk_all {
            x = to_coordinate(rand_rang.gen_range(0..SCREEN_WIDTH - 1));
            y = to_coordinate(rand_rang.gen_range(0..SCREEN_HEIGHT - 1));
            snk_all = self.snake_body.iter().all(|&snk| snk.x == x && snk.y == y);
        }
        self.food_x = x;
        self.food_y = y;
    }

    fn let_wall_cross(&mut self) {
        let s = self.snake_body.front_mut();
        // let mut snk_position = SnakePosition::new();
        match s {
            Some(snk) => {
                if snk.y < 0.0 {
                    snk.y = to_coordinate(SCREEN_HEIGHT);
                } else if snk.y > to_coordinate(SCREEN_HEIGHT) {
                    snk.y = 0.0;
                } else if snk.x < 0.0 {
                    snk.x = to_coordinate(SCREEN_WIDTH);
                } else if snk.x > to_coordinate(SCREEN_WIDTH - 1) {
                    snk.x = 0.0;
                }
            }
            None => (),
        }
    }

    fn move_forward(&mut self) {
        let mut snk_swap_helper_one = SnakePosition { x: 0.0, y: 0.0 };
        let mut snk_swap_helper_two = SnakePosition { x: 0.0, y: 0.0 };
        for (index, snk) in self.snake_body.iter_mut().enumerate() {
            // println!("{} {:?}", index, snk);
            if index == 0 {
                snk_swap_helper_one.x = snk.x;
                snk_swap_helper_one.y = snk.y;
                match self.direction {
                    SnakeDirection::Up => snk.y -= CORDINATE_INCREMENTOR,
                    SnakeDirection::Down => snk.y += CORDINATE_INCREMENTOR,
                    SnakeDirection::Right => snk.x += CORDINATE_INCREMENTOR,
                    SnakeDirection::Left => snk.x -= CORDINATE_INCREMENTOR,
                }
            } else {
                snk_swap_helper_two.x = snk.x;
                snk_swap_helper_two.y = snk.y;
                snk.x = snk_swap_helper_one.x;
                snk.y = snk_swap_helper_one.y;
                snk_swap_helper_one.x = snk_swap_helper_two.x;
                snk_swap_helper_one.y = snk_swap_helper_two.y;
            }
        }
    }

    fn add_snake_length(&mut self) {
        let snk_tail = self.snake_body.back().clone();
        match snk_tail {
            Some(snk) => {
                let snake = SnakePosition { x: snk.x, y: snk.y };
                self.snake_body.push_back(snake);
            }
            None => (),
        }
    }

    fn has_snake_touched_itself(&mut self) -> bool {
        let snake_head = self.snake_body.front();
        match snake_head {
            Some(snk) => {
                for (index, snk_body) in self.snake_body.iter().enumerate() {
                    if index > 0 && snk.x == snk_body.x && snk.y == snk_body.y {
                        return true;
                    }
                }
            }
            None => (),
        }
        return false;
    }

    fn restart_game(&mut self) {
        self.direction = SnakeDirection::Right;
        self.snake_body.clear();

        self.snake_body.push_back(SnakePosition {
            x: to_coordinate(2),
            y: to_coordinate(0),
        });

        self.snake_body.push_back(SnakePosition {
            x: to_coordinate(1),
            y: to_coordinate(0),
        });
        self.snake_body.push_back(SnakePosition {
            x: to_coordinate(0),
            y: to_coordinate(0),
        });

        self.food_x = to_coordinate(5);
        self.food_y = to_coordinate(5);
        self.waitime_time = 0.0;
        self.should_move = false;
        self.game_over = true;
    }
}
