use crate::entity::*;
use crate::vector::*;
pub trait SpawnComponent{
    fn process(&mut self)->Vec<Entity>;
}
pub struct BugSpawner{
    time_since_last_spawn:u32,
    y:u32
}
impl SpawnComponent for BugSpawner{
    fn process(&mut self)->Vec<Entity>{
        self.time_since_last_spawn+=1;
        self.y+=156329;
        if self.time_since_last_spawn> 80{
            self.time_since_last_spawn=0;
            vec![new_bug_entity(Vector2::new(2,(self.y%32) as i32))]
        }else{
            vec![]
        }
    }
}
impl BugSpawner{
    pub fn new()->Box<dyn SpawnComponent>{
        Box::new(BugSpawner{
            time_since_last_spawn:0,
            y:0,
        })
    }
}