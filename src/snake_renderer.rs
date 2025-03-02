use crate::display_out::DisplayOut;
use crate::gameplay::snake::Snake;

pub struct SnakeRenderer {
    pub display_out: DisplayOut,
    render_buffer: Vec<Vec<char>>
}

impl SnakeRenderer {
    pub fn render_frame(&mut self) {
        for i in 0..self.render_buffer.len() {
            for j in 0..self.render_buffer[i].len() {
                if i == 0 || i == self.render_buffer.len() - 1 {
                    self.render_buffer[i][j] = '#'; // Prints to top and bottom
                }
                if j == 0 || j == self.render_buffer[i].len() - 1 {
                     self.render_buffer[i][j] = '|'; // Prints Sides
                }
            }
        }


        DisplayOut::render_buffer(&self.render_buffer);
    }

    pub fn render_snake(&mut self, input: &Snake) -> Snake {
        let mut output = input.clone();
        for i in 0..output.snake_vec.len() {
            if output.snake_vec[i].x_pos >= self.display_out.width {
                output.snake_vec[i].x_pos = 0; // This will be temp
            }
            if output.snake_vec[i].y_pos >= self.display_out.height {
                output.snake_vec[i].y_pos = 0; // This will be tmp
            }
            if output.snake_vec[i].x_pos == 0 {
                output.snake_vec[i].x_pos = self.display_out.width - 1 ; // This will be temp
            }
            if output.snake_vec[i].y_pos == 0 {
                output.snake_vec[i].y_pos = self.display_out.height - 1; // This will be tmp
            }

            self.render_buffer[output.snake_vec[i].y_pos as usize][output.snake_vec[i].x_pos as usize] = '$';
        }
        output
    }

    pub fn clear_frame(&mut self) {
        self.display_out.clear_buffer(&mut self.render_buffer);
    }

    pub fn new(input_display: DisplayOut) -> SnakeRenderer {
        let buffer = input_display.create_buffer();
        SnakeRenderer {
            display_out: input_display,
            render_buffer: buffer,
        }
    }
}
