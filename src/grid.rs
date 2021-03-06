use crate::vector::*;
static TILE_SIZE: u32 = 20;
#[derive(Serialize, Deserialize, Clone, std::cmp::PartialEq, Debug)]
pub enum Tile {
    Glass,
    Background,
}

impl Tile {
    pub fn get_color(&self) -> u32 {
        match self {
            Self::Glass => 0xc4fffa,
            Self::Background => 0x000000,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Grid {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32, tiles: Vec<Tile>) -> Grid {
        assert_eq!(width*height,tiles.len() as u32);
        Grid {
            tiles: tiles,
            width: width,
            height: height,
        }
    }
    pub fn get_tile(&self, position: Vector2) -> Option<Tile> {
        let index_signed  = position.x *self.width as i32 + position.y;
        if index_signed<0{
            return None
        }
        let index = i32::abs(index_signed) as usize;
        if index < self.tiles.len() {
            return Some(self.tiles[index].clone());
        }
        None
    }
    pub fn in_range(&self,position: Vector2)->bool{
        if position.x>=0 && position.x<self.width as i32 && position.y>=0 && position.y<self.height as i32{
            return true;
        }else{
            return false;
        }
    }
    pub fn draw(&self) -> Vec<u32> {
        let mut out: Vec<u32> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                out.append(&mut vec![
                    self.tiles[(y * self.width + x) as usize].get_color(),
                    x * TILE_SIZE,
                    y * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                ])
            }
        }
        return out;
    }
}
