

use crate::game_state::Direction;

#[derive(Debug)]
pub struct Snake {
    pub snake_vec: Vec<SnakeSegment>,
}

impl Snake {
    pub fn new(init_x: u32, init_y: u32, init_direction: Direction, init_len: u32) -> Snake {
        let root_segment = SnakeSegment {
            direction: init_direction,
            x_pos: init_x,
            y_pos: init_y,
        };

        let mut output = Snake {
            snake_vec: Vec::new(),
        };

        output.snake_vec.push(root_segment);

        for _ in 0..init_len {
            output.add_segment();
        }
        output
    }

    pub fn move_direction(&mut self, input: Direction) {
        match input {
            Direction::Up => {
                let tmp = self.snake_vec[0];
                self.snake_vec[0].y_pos += 1;
                self.propogate_postition(tmp, 1);
            }
            Direction::Down => {
                let tmp = self.snake_vec[0];
                self.snake_vec[0].y_pos -= 1;
                self.propogate_postition(tmp, 1);
            }
            Direction::Left => {
                let tmp = self.snake_vec[0];
                self.snake_vec[0].x_pos -= 1;
                self.propogate_postition(tmp, 1);
            }
            Direction::Right => {
                let tmp = self.snake_vec[0];
                self.snake_vec[0].x_pos += 1;
                self.propogate_postition(tmp, 1);
            }
        }

    }

    fn propogate_postition(&mut self, input: SnakeSegment, index: usize) {
        if index == self.snake_vec.len() {
            return;
        }
        let tmp = self.snake_vec[index];

        self.snake_vec[index] = input;
        self.propogate_postition(tmp, index + 1);
    }

    pub fn add_segment(&mut self) {
        let last_segment = self.snake_vec.pop().unwrap();


        let mut new_segment = SnakeSegment {
            direction: last_segment.direction.clone(),
            x_pos: 0,
            y_pos: 0,
        };

        match last_segment.direction {
            Direction::Up => {
                new_segment.x_pos = last_segment.x_pos;
                new_segment.y_pos = last_segment.y_pos - 1;
            }
            Direction::Down => {
                new_segment.x_pos = last_segment.x_pos;
                new_segment.y_pos = last_segment.y_pos + 1;
            }
            Direction::Left => {
                new_segment.x_pos = last_segment.x_pos + 1;
                new_segment.y_pos = last_segment.y_pos;
            }
            Direction::Right => {
                new_segment.x_pos = last_segment.x_pos - 1;
                new_segment.y_pos = last_segment.y_pos;
            }
        }

        self.snake_vec.push(last_segment);
        self.snake_vec.push(new_segment);

    }
}

#[derive(Debug, Copy, Clone)]
pub struct SnakeSegment {
    pub direction: Direction,
    pub x_pos: u32,
    pub y_pos: u32,
}

