use crate::vector::*;
use crate::controller::*;
static TILE_SIZE: u32 = 20;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EntityTeam {
    Player,
    Enemy,
    Food,
    Snake,
    Fire,
    Bug,
}
#[derive(Debug, Clone)]
pub struct Entity {
    state: EntityState,
    components: Vec<Box<dyn Component>>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EntityState {
    pub position: Vector2,
    pub delta_position: Vector2,
    pub health: u32,
    pub max_health: u32,
    pub base_color: u32,
    pub team: EntityTeam,
    pub dead: bool,
}
impl Entity {
    pub fn new(
        pos: Vector2,
        health: u32,
        max_health: u32,
        base_color: u32,
        team: EntityTeam,
        components: Vec<Box<dyn Component>>,
    ) -> Entity {
        Entity {
            state: EntityState {
                position: pos,
                delta_position: Vector2::new(0, 0),
                health: health,
                max_health: max_health,
                base_color: base_color,
                team: team,
                dead: false,
            },
            components: components,
        }
    }
    pub fn draw(&self) -> Vec<u32> {
        let health = (self.state.max_health as f64 - self.state.health as f64)
            / (self.state.max_health as f64);
        let current_red = (self.state.base_color >> 16) & 0x0000ff;
        let red = (((0xff - current_red) as f64) * health) as u32 & 0x0000ff;
        let current_green = (self.state.base_color & 0x00ff00) >> 8;
        let green = (((0xff - current_green) as f64) * health) as u32;
        let current_blue = self.state.base_color & 0x0000ff;
        let blue = (((0xff - current_blue) as f64) * health) as u32;
        vec![
            (red << 16) + (green << 8) + blue + self.state.base_color,
            (self.state.position.x as u32 * TILE_SIZE) as u32,
            (self.state.position.y as u32 * TILE_SIZE) as u32,
            TILE_SIZE,
            TILE_SIZE,
        ]
    }
    pub fn process(
        &mut self,
        input: &Controller,
        grid: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        let mut entity_vec = vec![];
        for component in self.components.iter_mut() {
            entity_vec.append(&mut component.process(input, &mut self.state, grid, entities));
        }
        return entity_vec;
    }
    pub fn get_position(&self) -> Vector2 {
        self.state.position.clone()
    }
    pub fn get_dead(&self)->bool{
        self.state.dead
    }
}
#[derive(Debug, Clone)]
pub struct EntityGrid{
    //first vec is a matrix of each tile point
    grid:Vec<Vec<Entity>>,
    width:usize,
    height:usize,
}
impl EntityGrid{
    pub fn new(width:usize,height:usize)->EntityGrid{
        EntityGrid{
            width:width,
            height:height,
            grid:vec![vec![];width as usize *height as usize],
        }
    }
    pub fn add_entity(&mut self,entity:&Entity){
        let index = self.get_index_vector(entity.get_position());
        if index<self.grid.len(){
            self.grid[index].push(entity.clone());
        }
    }
    fn get_index(&self,x:usize,y:usize)->usize{
        y*self.width+x
    }
    fn get_index_vector(&self,position:Vector2)->usize{
        let x = i32::abs(position.x) as usize;
        let y = i32::abs(position.y) as usize;
        self.get_index(x,y)
    }
    pub fn add_entity_vec(&mut self,entity_vec:&Vec<Entity>){
        for entity in entity_vec.iter(){
            self.add_entity(&entity);
        }
    }
    pub fn get_entities(&mut self,position:Vector2)->Vec<Entity>{
        let index = self.get_index_vector(position);
        if index<self.grid.len(){
            self.grid[index].clone()
        }else{
            vec![]
        }
    }
    pub fn get_entities_mut(&mut self,position:Vector2)->Option<&mut Vec<Entity>>{
        let index = self.get_index_vector(position);
        if index<self.grid.len(){
            Some(&mut self.grid[index])
        }else{
            None
        }
    }
    pub fn update_entity_position(&mut self){
        let mut new_grid = EntityGrid::new(self.width, self.height);
        for vec in self.grid.iter(){
            new_grid.add_entity_vec(vec);
        }
        self.grid = new_grid.grid;
    }
}
pub trait Component: std::fmt::Debug{
    fn process(
        &mut self,
        user_input: &Controller,
        state: &mut EntityState,
        world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity>;
    fn box_clone(&self) -> Box<dyn Component>;
}
impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
#[derive(Debug, Clone)]
pub struct InputComponent {}
impl InputComponent {
    pub fn new() -> Box<dyn Component> {
        Box::new(InputComponent {})
    }
}
impl Component for InputComponent {
    fn process(
        &mut self,
        user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        _entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        state.delta_position = user_input.get_main_axis().clone();
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}

#[derive(Debug, Clone)]
pub struct GridComponent {}
impl Component for GridComponent {
    fn process(
        &mut self,
        _user_input: &Controller,
        state: &mut EntityState,
        world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        if let Some(tile) = world.get_tile(state.position.clone() + state.delta_position.clone()) {
            if tile != crate::grid::Tile::Glass {
                let mut no_entity_found = true;
                let new_pos = state.position.clone()+state.delta_position.clone();

                for entitity in entities.iter(){
                    if entitity.get_position()==new_pos{
                        no_entity_found=false;
                    }
                }
                if no_entity_found && world.in_range(new_pos){
                    state.position += state.delta_position.clone();
                }
            }
            state.delta_position = Vector2::new(0, 0);
            vec![]
        } else {
            state.delta_position = Vector2::new(0, 0);
            vec![]
        }
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl GridComponent {
    pub fn new() -> Box<dyn Component> {
        Box::new(GridComponent {})
    }
}
#[derive(Debug, Clone)]
pub struct EnemyDamageComponent {}
impl Component for EnemyDamageComponent {
    fn process(
        &mut self,
        _user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        if state.health == 0 {
            state.delta_position = Vector2::new(0, 0);
            state.dead = true;
        }
        let pos = state.position.clone() + state.delta_position.clone();
        for ent in entities.iter() {
            if ent.state.position == pos && ent.state.team != state.team && state.health > 0 {
                state.health -= 1;
                state.delta_position = Vector2::new(0, 0);
            }
        }
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}

impl EnemyDamageComponent {
    pub fn new() -> Box<dyn Component> {
        Box::new(EnemyDamageComponent {})
    }
}
#[derive(Debug, Clone)]
pub struct GravityComponent {
    ticker: u32,
    fall_time: u32, //number of frames before Gravity component falls one unit
}
impl Component for GravityComponent {
    fn process(
        &mut self,
        _user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        _entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        self.ticker += 1;
        if self.ticker > self.fall_time {
            state.delta_position.y += 1;
            self.ticker = 0;
        }
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}

impl GravityComponent {
    pub fn new() -> Box<dyn Component> {
        Box::new(GravityComponent {
            ticker: 0,
            fall_time: 4,
        })
    }
}
#[derive(Debug, Clone)]
pub struct SnakeBodyComponent {
    cool_down: u32,
}
impl Component for SnakeBodyComponent {
    fn process(
        &mut self,
        _user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        if self.cool_down < 10000 {
            self.cool_down += 1;
        }
        for ent in entities.iter() {
            if ent.state.team == EntityTeam::Food
                && state.position.within_one_of(&ent.state.position)
            {
                if self.cool_down > 100 {

                    let mut no_snake_at_pos = true;
                    for ent2 in entities.iter(){
                        if ent2.state.position==ent.state.position && ent2.state.team!=ent.state.team{
                            no_snake_at_pos=false;
                        }
                    }
                    if no_snake_at_pos{
                        self.cool_down = 0;
                        return vec![new_snake_entity(ent.state.position.clone())];
                    }
                }
            }
        }
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl SnakeBodyComponent {
    pub fn new() -> Box<dyn Component> {
        Box::new(SnakeBodyComponent { cool_down: 0 })
    }
}
#[derive(Debug, Clone)]
pub struct SpawnComponent{
    cool_down: u32,
    spawn_fn:fn(Vector2)->Entity,
}

impl Component for SpawnComponent{
    fn process(
        &mut self,
        user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        _entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        if self.cool_down < 10000 {
            self.cool_down += 1;
        }
        for input in user_input.get_buttons().iter(){
            if input==&" ".to_string() && self.cool_down>100{
                self.cool_down=0;
                let test_fn = self.spawn_fn;
                return vec![test_fn(state.position.clone())]
            }
        }
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl SpawnComponent{
    pub fn new(cool_down:u32,spawn_fn:fn(Vector2)->Entity)->Box<dyn Component>{
        Box::new(SpawnComponent{
            cool_down:cool_down,
            spawn_fn:spawn_fn
        })
    }
}
#[derive(Debug, Clone)]
struct LifetimeComponent{
    current_lived:u64,
    lifespan:u64,
}
impl Component for LifetimeComponent{
    fn process(
        &mut self,
        _user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        _entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        self.current_lived+=1;
        if self.current_lived>self.lifespan{
            state.dead=true;
        }
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl LifetimeComponent{
    pub fn new(lifespan:u64)->Box<dyn Component>{
        Box::new(LifetimeComponent{
            current_lived:0,
            lifespan:lifespan,
        })
    }
}
#[derive(Debug, Clone)]
struct SteamComponent{
    time_since_last_last_rise:u64,
    rain_height:i32,
    rise_time:u64,
}
impl Component for SteamComponent{
    fn process(
        &mut self,
        _user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        _entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        self.time_since_last_last_rise+=1;
        if self.time_since_last_last_rise>self.rise_time{
            state.delta_position.y-=1;
            self.time_since_last_last_rise=0;
        }
        if state.position.y<self.rain_height{
            state.dead=true;
            return vec![new_water_entity(state.position.clone())];
        }
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl SteamComponent{
    pub fn new(rise_time:u64,rain_height:i32)->Box<dyn Component>{
        Box::new(SteamComponent{
            time_since_last_last_rise:0,
            rise_time:rise_time,
            rain_height:rain_height,
        })
    }
}
#[derive(Debug, Clone)]
struct WaterComponent{
    water_lifetime:u32,
    time_lived:u32,
}
impl Component for WaterComponent{
    fn process(
        &mut self,
        _user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        self.time_lived+=1;
        if WaterComponent::check_point(state.position.clone()+Vector2::new(0,-1), entities)==false && self.time_lived>self.water_lifetime{
            state.dead=true;
            return vec![new_steam_entity(state.position.clone()+Vector2::new(0,-1))]
        }
        if WaterComponent::check_point(state.position.clone()+Vector2::new(0,1), entities)==false{
            state.delta_position=Vector2::new(0,1);
            return vec![]
        }
        if WaterComponent::check_point(state.position.clone()+Vector2::new(1,1), entities)==false{
            state.delta_position=Vector2::new(1,1);
            return vec![]
        }
        if WaterComponent::check_point(state.position.clone()+Vector2::new(-1,1), entities)==false{
            state.delta_position=Vector2::new(-1,1);
            return vec![]
        }
        if WaterComponent::check_point(state.position.clone()+Vector2::new(-1,0), entities)==false{
            state.delta_position=Vector2::new(-1,0);
            return vec![]
        }
        if WaterComponent::check_point(state.position.clone()+Vector2::new(1,0), entities)==false{
            state.delta_position=Vector2::new(1,0);
            return vec![]
        }

        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl WaterComponent{
    pub fn new(water_lifetime:u32)->Box<dyn Component>{
        Box::new(WaterComponent{
            water_lifetime:water_lifetime,
            time_lived:0,
        })
    }
    fn check_point(pos:Vector2,entities:&Vec<Entity>)->bool{
        for entity in entities.iter(){
            if pos==entity.get_position(){
                return true;
            }
        }
        return false;
    }
}
#[derive(Debug, Clone)]
struct PlantComponent{
    time_since_last_last_grow:u64,
    grow_time:u64,
    done_growing:bool
}
impl Component for PlantComponent{
    fn process(
        &mut self,
        _user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        self.time_since_last_last_grow+=1;
        for ent in entities.iter(){
            if ent.state.team==EntityTeam::Bug && state.position.within_one_of(&ent.get_position()){
               state.dead=true;
               return vec![] 
            }
        }
        if self.time_since_last_last_grow>self.grow_time && self.done_growing == false{
            self.time_since_last_last_grow=0;
            self.done_growing=true;
            vec![new_plant_entity(state.position.clone()+Vector2::new(0,-1))]
        }else{
            vec![]
        }
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl PlantComponent{
    pub fn new(grow_time:u64)->Box<dyn Component>{
        Box::new(PlantComponent{
            time_since_last_last_grow:0,
            grow_time:grow_time,
            done_growing:false,
        })
    }
}
#[derive(Debug, Clone)]
struct FireComponent{
    time_since_last_last_expand:u64,
    grow_time:u64,
    growing_countdown:u32,
    time_alive:u32,
}
impl Component for FireComponent{
    fn process(
        &mut self,
        _user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        _entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        self.time_since_last_last_expand+=1;
        self.time_alive+=1;
        if self.time_alive>100{
            state.dead=true;
        }
        if self.time_since_last_last_expand>self.grow_time && self.growing_countdown>0{
            self.time_since_last_last_expand=0;
            let growing_countdown = self.growing_countdown;
            self.growing_countdown=0;
            vec![new_fire_entity_countdown(state.position.clone()+Vector2::new(0,-1),growing_countdown-1),
            new_fire_entity_countdown(state.position.clone()+Vector2::new(0, 1),growing_countdown-1),
            new_fire_entity_countdown(state.position.clone()+Vector2::new(1, 0),growing_countdown-1),
            new_fire_entity_countdown(state.position.clone()+Vector2::new(-1,0),growing_countdown-1)]
        }else{
            vec![]
        }
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl FireComponent{
    pub fn new(grow_time:u64,countdown:u32)->Box<dyn Component>{
        Box::new(FireComponent{
            time_alive:0,
            time_since_last_last_expand:0,
            grow_time:grow_time,
            
            growing_countdown:countdown,
        })
    }
}
#[derive(Debug, Clone)]
struct BugComponent{
    time_since_last_last_expand:u64,
    grow_time:u64,
}
impl Component for BugComponent{
    fn process(
        &mut self,
        _user_input: &Controller,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        self.time_since_last_last_expand+=1;
        for ent in entities.iter(){
            if ent.state.team==EntityTeam::Fire && state.position.within_one_of(&ent.get_position()){
               state.dead=true;
               return vec![] 
            }
        }
        if self.time_since_last_last_expand>self.grow_time{
            self.time_since_last_last_expand=0;
            state.delta_position+=Vector2::new(1, 0);
            vec![]
        }else{
            vec![]
        }
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl BugComponent{
    pub fn new(grow_time:u64)->Box<dyn Component>{
        Box::new(BugComponent{
            time_since_last_last_expand:0,
            grow_time:grow_time,
        })
    }
}
pub fn new_bug_entity(position:Vector2)->Entity{
    Entity::new(position, 1, 1, 0x00ffff, EntityTeam::Bug, vec![BugComponent::new(80),GridComponent::new()])
}
pub fn new_fire_entity_countdown(position:Vector2,countdown:u32)->Entity{
    Entity::new(position, 1, 1, 0xff0000, EntityTeam::Fire, vec![FireComponent::new(15,countdown),GridComponent::new()])

}
pub fn new_fire_entity(position:Vector2)->Entity{
    Entity::new(position, 1, 1, 0xff0000, EntityTeam::Food, vec![FireComponent::new(15,3),GridComponent::new()])

}
pub fn new_water_entity(position:Vector2)->Entity{
    Entity::new(position, 1, 1, 0x0042ff, EntityTeam::Food, vec![WaterComponent::new(300),GridComponent::new()])
}
pub fn new_steam_entity(position:Vector2)->Entity{
    Entity::new(position, 1, 1, 0xc2fffc, EntityTeam::Food, vec![SteamComponent::new(40,5),GridComponent::new()])
}
pub fn new_plant_entity(position:Vector2)->Entity{
    Entity::new(position, 1, 1, 0x00aa00, EntityTeam::Food, vec![PlantComponent::new(40),GridComponent::new()])
}
pub fn new_snake_entity(position: Vector2) -> Entity {
    Entity::new(
        position,
        1,
        1,
        0x007b12,
        EntityTeam::Snake,
        vec![SnakeBodyComponent::new(), GridComponent::new()],
    )
}
pub fn new_food(position: Vector2) -> Entity {
    Entity::new(
        position,
        10,
        10,
        0xffef00,
        EntityTeam::Food,
        vec![
            GravityComponent::new(),
            GridComponent::new(),
            LifetimeComponent::new(850),
        ],
    )
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn player_draw() {
        let mut p = Entity::new(
            Vector2::new(0, 0),
            10,
            10,
            0x00ff00,
            EntityTeam::Player,
            vec![
                InputComponent::new(),
                GridComponent::new(),
                EnemyDamageComponent::new(),
            ],
        );
        assert_eq!(p.draw(), vec![0x00ff00, 0, 0 as u32, TILE_SIZE, TILE_SIZE]);
        p.state.health = 0;
        assert_eq!(p.draw(), vec![0xffffff, 0, 0 as u32, TILE_SIZE, TILE_SIZE]);
    }
    #[test]
    fn player_process_draw() {
        let p = Entity::new(
            Vector2::new(0, 0),
            10,
            10,
            0x00ff00,
            EntityTeam::Player,
            vec![
                InputComponent::new(),
                GridComponent::new(),
                EnemyDamageComponent::new(),
            ],
        );
        p.draw();
    }
    #[test]
    fn player_empty_process() {
        let mut e = Entity::new(
            Vector2::new(0, 0),
            10,
            10,
            0x00ff00,
            EntityTeam::Player,
            vec![
                InputComponent::new(),
                GridComponent::new(),
                EnemyDamageComponent::new(),
            ],
        );
        e.process(
            &Controller::new(Vector2::new(0, 0),&vec![]),
            &crate::grid::Grid::new(0, 0, vec![]),
            &vec![],
        );
    }
    #[test]
    fn component_clone() {
        let c: Box<dyn Component> = InputComponent::new();
        let _c2 = c.clone();
    }
    #[test]
    fn entity_clone() {
        let e = Entity::new(
            Vector2::new(0, 0),
            10,
            10,
            0x00ff00,
            EntityTeam::Player,
            vec![InputComponent::new()],
        );
        let _e2 = e.clone();
    }
    #[test]
    fn entity_grid_constructor(){
        let e = EntityGrid::new(1,1);
        assert_eq!(e.width,1);
        assert_eq!(e.height,1);
        assert_eq!(e.grid.len(),1);
    }
    #[test]
    fn add_and_get_entity(){
        let mut e = EntityGrid::new(2,2);
        e.add_entity(&new_food(Vector2::new(0,0)));
        e.add_entity(&new_food(Vector2::new(1,0)));
        assert_eq!(e.get_entities(Vector2::new(0,0)).len(),1);
        assert_eq!(e.get_entities(Vector2::new(1,0)).len(),1);
        assert_eq!(e.get_entities(Vector2::new(1,1)).len(),0);
    }
    #[test]
    fn add_entity_vec(){
        let mut e = EntityGrid::new(2,2);
        e.add_entity_vec(&vec![new_food(Vector2::new(0,0)),new_food(Vector2::new(0,0)),new_food(Vector2::new(0,1))]);
        assert_eq!(e.get_entities(Vector2::new(0,0)).len(),2);
        assert_eq!(e.get_entities(Vector2::new(0,1)).len(),1);
    }
    #[test]
    fn get_entities_mut(){
        let mut e = EntityGrid::new(1,1);
        e.add_entity_vec(&vec![new_food(Vector2::new(0,0)),new_food(Vector2::new(0,0))]);
        assert_eq!(e.get_entities_mut(Vector2::new(0,0)).unwrap().len(),2);
    }
    #[test]
    fn update_position(){
        let mut e = EntityGrid::new(2,2);
        e.add_entity_vec(&vec![new_food(Vector2::new(0,0))]);
        {
            let e_vec = e.get_entities_mut(Vector2::new(0, 0)).unwrap();
            e_vec[0].state.position=Vector2::new(1,1);
            assert_eq!(e_vec[0].get_position(),Vector2::new(1, 1)); 
        }
        e.update_entity_position();
        assert_eq!(e.get_entities(Vector2::new(0,0)).len(),0);
        assert_eq!(e.get_entities(Vector2::new(1,1)).len(),1);
    }
}
