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
        let mut plant_alive = false;
        for entity in entities.iter(){
            if entity.get_team()==EntityTeam::Plant{
                plant_alive=true;
            }
        }
        if plant_alive==false{
            return (true,StateCommand::Push(LooseState::new()))
        }else{
            return (false,StateCommand::NoAction)
        }
    }
}
impl BugWatcher{
    pub fn new()->Box<dyn StateObserver>{
        Box::new(BugWatcher{})
    }
}