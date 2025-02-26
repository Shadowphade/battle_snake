use crate::display_out::DisplayOut;
use std::time::Instant;

pub struct SnakeRenderer {
    display_out: DisplayOut,
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
    pub fn new(input_display: DisplayOut) -> SnakeRenderer {
        let buffer = input_display.create_buffer();
        SnakeRenderer {
            display_out: input_display,
            render_buffer: buffer,
        }
    }
}
