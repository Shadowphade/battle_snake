use std::io::{self, Write};
use std::{thread, time};

pub struct DisplayOut {
    pub width: u32,
    pub height: u32,
}

impl DisplayOut {
    pub fn create_buffer(&self) -> Vec<Vec<char>> {
        let mut output = Vec::new();
        for _ in 0..self.height {
            let mut x_vec = Vec::new();
            for _ in 0..self.width{
                let defaut_char = ' ';
                x_vec.push(defaut_char);
            }
            output.push(x_vec);
        }
        output
    }

    pub fn clear_buffer(&self, input_buffer: &mut Vec<Vec<char>>) {
        for i in 0..input_buffer.len() {
            for j in 0..input_buffer[i].len() {
                input_buffer[i][j] = ' ';
            }
        }
    }

    pub fn render_buffer(input_buffer: &Vec<Vec<char>>) {
        // VERY TEMPORARY SLEEP
        let sleep_time = time::Duration::from_millis(10);

        thread::sleep(sleep_time);

        let output = io::stdout();
        let mut output_handle = output.lock();

        //output_handle.write_all(b"\x1B[2J").unwrap();

        for i in input_buffer {
            for j in i {
                print!("{}", j);
            }
            println!();
        }
        output_handle.write_all(b"\x1B[H").unwrap();
    }
}
