use crate::snake_renderer::SnakeRenderer;
use crate::display_out::DisplayOut;
use crate::gameplay::snake::Snake;
use crate::gameplay::ai::Ai;

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
        let direction = Direction::Right;
        let mut snake = Snake::new(middle_x, middle_y, direction, 3);
        let mut ai = Ai::new(crate::gameplay::ai::AiType::Random);

        let mut frame_timer = 0;
        loop {

            let ai_direction = ai.run(&snake);
            snake.move_direction(ai_direction);

            snake = self.renderer.render_snake(&snake);
            self.renderer.render_frame();
            self.renderer.clear_frame();
            if frame_timer % 10 == 0{
                snake.add_segment();

            }
            //println!("{:?}", snake);

            frame_timer += 1;
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
