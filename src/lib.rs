extern crate wasm_bindgen;
#[allow(unused_imports)]
use serde_wasm_bindgen::*;
#[macro_use]
extern crate serde_derive;
use wasm_bindgen::prelude::*;
extern crate wee_alloc;
mod vector;
use vector::*;
mod entity;
use entity::*;
mod controller;
use controller::*;
mod spawn;
mod loose_state;
mod state;
use state::*;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[allow(dead_code)]
static TILE_SIZE: u32 = 20;
mod grid;
use grid::*;
mod state_observer;
use state_observer::*;
#[wasm_bindgen]
pub struct StateStack{
    states:Vec<Box<dyn State>>
}
#[wasm_bindgen]
impl StateStack{
    pub fn game_loop(&mut self, input: Controller)->Vec<u32>{
        let len = self.states.len()-1;
        let (draw,command) = self.states[len].game_loop(input);
        match command{
            StateCommand::NoAction=>(),
            StateCommand::Push(state)=>self.states.push(state),
            StateCommand::Pop=>{self.states.pop();},
        };
        draw
    }
    pub fn game_loop_js(&mut self, input: JsValue) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self.game_loop(serde_wasm_bindgen::from_value(input).ok().unwrap()),
        )
        .ok()
        .unwrap()
    }
    fn new(states:Vec<Box<dyn State>>)->StateStack{
        StateStack{
            states:states
        }
    }
}
#[wasm_bindgen]
pub struct PlayState {
    entities: Vec<Entity>,
    spawners: Vec<Box<dyn spawn::SpawnComponent>>,
    grid: Grid,
    observers: Vec<Box<dyn StateObserver>>,
}
#[wasm_bindgen]
impl PlayState {
    fn process(&mut self, input: Controller)->StateCommand{
        for spawn in self.spawners.iter_mut(){
            self.entities.append(&mut spawn.process());
        }
        let old_entities = &self.entities.clone();
        let mut new_entities = vec![];

        for entity in self.entities.iter_mut() {
            for ent in entity.process(&input, &self.grid, old_entities).iter(){
                if self.grid.in_range(ent.get_position()){
                    new_entities.push(ent.clone());
                }
            }
        }
        self.entities.append(&mut new_entities);
        self.kill_dead();
        for observer in self.observers.iter_mut(){
            let (cont,command) = observer.process(&self.entities);
            if cont{
                return command
            }
        }
        StateCommand::NoAction
    }
    pub fn draw(&self) -> Vec<u32> {
        let mut draws = self.grid.draw();
        for ent in self.entities.iter() {
            draws.append(&mut ent.draw());
        }
        return draws;
    }
    pub fn kill_dead(&mut self){
        let mut new_entities = vec![];
        for entity in self.entities.iter(){
            if entity.get_dead()==false{
                new_entities.push(entity.clone());
            }
        }
        self.entities = new_entities;
    }
    pub fn game_loop_js(&mut self, input: JsValue) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self.game_loop(serde_wasm_bindgen::from_value(input).ok().unwrap()),
        )
        .ok()
        .unwrap()
    }
    pub fn game_loop(&mut self, input: Controller) -> Vec<u32> {
        self.process(input);
        self.draw()
    }
    #[allow(dead_code)]
    fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}
impl State for PlayState{
    fn process(&mut self, input: Controller)->StateCommand{
        self.process(input)
    }
    fn draw(&self)->Vec<u32>{
        self.draw()
    }
}
pub struct MainOutput {
    pub state: PlayState,
    pub draw_calls: Vec<u32>,
}

fn new_cursor(position: Vector2) -> Entity {
    Entity::new(
        position,
        10,
        10,
        0x00ff00,
        EntityTeam::Player,
        vec![
            entity::InputComponent::new(),
            entity::GridComponent::new(),
            entity::SpawnComponent::new(10,new_fire_entity),
        ],
    )
}
#[allow(dead_code)]
fn new_enemy(position: Vector2) -> Entity {
    Entity::new(
        position,
        10,
        10,
        0xff0000,
        EntityTeam::Enemy,
        vec![
            entity::InputComponent::new(),
            entity::GridComponent::new(),
            entity::EnemyDamageComponent::new(),
        ],
    )
}
#[allow(dead_code)]
fn new_prize(position: Vector2) -> Entity {
    Entity::new(
        position,
        10,
        10,
        0xffec00,
        EntityTeam::Player,
        vec![entity::GridComponent::new()],
    )
}

pub fn init_state() -> StateStack {
    let mut map = vec![Tile::Background;32*32];
    let state = PlayState {
        entities: vec![
            new_cursor(Vector2::new(2, 3)),
            new_plant_entity(Vector2::new(16, 31)),
        ],
        grid: Grid::new(32, 32, map),
        spawners: vec![spawn::BugSpawner::new()],
        observers:vec![BugWatcher::new()]
    };
    StateStack::new(vec![Box::new(state)])
}
#[wasm_bindgen]
pub fn init_state_js() -> StateStack {
    init_state()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_grid() {
        let v: Vec<Tile> = vec![];
        let g = Grid::new(0, 0, v);
        assert!(g.draw().len() == 0)
    }
    #[test]
    fn one_by_one_grid() {
        let g = Grid::new(1, 1, vec![Tile::Glass]);
        assert_eq!(
            g.draw(),
            vec![Tile::Glass.get_color(), 0, 0, TILE_SIZE, TILE_SIZE]
        )
    }
    #[test]
    fn test_init_state() {
        init_state();
    }
    #[test]
    fn run_frame() {
        let mut s = init_state();
        s.game_loop(Controller::new(Vector2::new(0, 0),&vec![]));
    }
}
