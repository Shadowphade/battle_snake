use crate::snake_renderer::SnakeRenderer;
use crate::display_out::DisplayOut;

pub struct GameState {
    pub score: u32,
    renderer: SnakeRenderer,
}

impl GameState {
    pub fn run(&mut self) {
        loop {
            self.renderer.render_frame();
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
