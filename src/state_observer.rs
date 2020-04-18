use crate::state::*;
use crate::entity::*;
use crate::loose_state::*;
pub trait StateObserver{
    fn process(&mut self,entities:&Vec<Entity>)->(bool,StateCommand);
}
pub struct BugWatcher{

}
impl StateObserver for BugWatcher{
    fn process(&mut self,entities:&Vec<Entity>)->(bool,StateCommand){
        for entity in entities.iter(){
            if entity.get_position().x>25 && entity.get_team()==EntityTeam::Bug{
                return (true,StateCommand::Push(LooseState::new()))
            }
        }
        return (false,StateCommand::NoAction);
    }
}
impl BugWatcher{
    pub fn new()->Box<dyn StateObserver>{
        Box::new(BugWatcher{})
    }
}