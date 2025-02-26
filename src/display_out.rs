use std::io::{self, Write};
use std::{thread, time};

pub struct DisplayOut {
    pub width: u32,
    pub height: u32,
}

impl DisplayOut {
    pub fn create_buffer(&self) -> Vec<Vec<char>> {
        let mut output = Vec::new();
        for _ in 0..self.width {
            let mut y_vec = Vec::new();
            for _ in 0..self.height{
                let defaut_char = ' ';
                y_vec.push(defaut_char);
            }
            output.push(y_vec);
        }
        output
    }

    pub fn render_buffer(input_buffer: &Vec<Vec<char>>) {
        // VERY TEMPORARY SLEEP
        let sleep_time = time::Duration::from_millis(10);

        thread::sleep(sleep_time);

        let output = io::stdout();
        let mut output_handle = output.lock();

        output_handle.write_all(b"\x1B[2J").unwrap();

        let total_render_chars = input_buffer.len() * input_buffer[0].len();

        let mut index = 0;
        let mut jindx = 0;
        for _ in 0..total_render_chars {
            if index == input_buffer.len() {
                println!();
                index = 0;
                jindx += 1;
            }
            if jindx == input_buffer[0].len() {
                println!("Set thing to zero");
                jindx = 0;
            }
            print!("{}", input_buffer[index][jindx]);
            index += 1;

        }
        output_handle.write_all(b"\x1B[H").unwrap();
    }
}
