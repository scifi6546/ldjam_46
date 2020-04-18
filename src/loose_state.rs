use crate::grid::*;
use crate::state::*;
use crate::controller::*;
pub struct LooseState{
    grid:Grid,
}
impl State for LooseState{
    fn draw(&self)->Vec<u32>{
        self.grid.draw()
    }
    fn process(&mut self, input: Controller)->StateCommand{
        StateCommand::NoAction
    }
}
impl LooseState{
    pub fn new()->Box<dyn State>{
        Box::new(LooseState{
            grid: Grid::new(1, 1, vec![Tile::Background])
        })
    }
}