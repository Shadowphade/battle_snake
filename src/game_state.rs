use crate::snake_renderer::SnakeRenderer;
use crate::display_out::DisplayOut;
use crate::gameplay::snake::Snake;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct GameState {
    pub score: u32,
    renderer: SnakeRenderer,
}

impl GameState {
    pub fn run(&mut self) {
        let middle_x = self.renderer.display_out.width / 2;
        let middle_y = self.renderer.display_out.height / 2;
        let direction = Direction::Left;
        let mut snake = Snake::new(middle_x, middle_y, direction, 3);
        let mut direction_changer = 1;
        loop {
            snake.move_direction(snake.snake_vec[0].direction);
            self.renderer.render_snake(&mut snake);
            self.renderer.render_frame();
            self.renderer.clear_frame();
            if direction_changer % 5 == 0 {
                snake.move_direction(Direction::Up);
                snake.add_segment();
            }

            direction_changer += 1;

        }
    }

    pub fn new(width: u32, height: u32) -> GameState {
        let output = GameState {
            score: 0,
            renderer: SnakeRenderer::new(DisplayOut { width, height}),
        };
        output
    }
}
