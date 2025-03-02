
use rand::Rng;

pub struct Apple {
    pub x_pos: u32,
    pub y_pos: u32,
}

impl Apple {
    pub fn new(max_x: u32, max_y: u32) -> Apple{
        let mut rng = rand::rng();
        let output = Apple {
            x_pos: rng.random_range(0..=max_x),
            y_pos: rng.random_range(0..=max_y),

        };
        return output;
    }
}
