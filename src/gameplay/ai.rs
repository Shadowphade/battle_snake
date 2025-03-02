use rand::Rng;
use crate::gameplay::snake::Snake;
use crate::game_state::Direction;

pub struct Ai {
    ai_type: AiType,
}

#[derive(Debug, Copy, Clone)]
pub enum AiType {
    Random,
    Dijkstra,
    Astar,
}

impl Ai {
    pub fn new(ai_type: AiType) -> Ai{
        let output =  Ai {
            ai_type,
        };
        output
    }

    pub fn run(&mut self, _snake: &Snake) -> Direction {
        match self.ai_type {
            AiType::Random => {
                return self.random();
            }
            AiType::Dijkstra => {
                return Direction::Down;
            }
            AiType::Astar => {
                return Direction::Down;
            }
        }
    }

    fn random(&mut self) -> Direction{
        let mut rng = rand::rng();
        let direction = rng.random_range(0..=3);
        if direction == 0 {
            return Direction::Up;
        }
        if direction == 1 {
            return Direction::Down;
        }
        if direction == 2 {
            return Direction::Left;
        }
        if direction == 3 {
            return Direction::Right;
        }
        Direction::Up
    }
}
