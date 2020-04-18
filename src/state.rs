use crate::controller::*;
pub enum StateCommand{
    NoAction,
    Push(Box<dyn State>),
    Pop,
}

pub trait State{
    fn game_loop(&mut self, input: Controller) -> (Vec<u32>, StateCommand) {
        let res = self.process(input);
        (self.draw(),res)
    }
    fn draw(&self) -> Vec<u32>;
    fn process(&mut self, input: Controller)->StateCommand;
}