use crate::ecs::{GenManager,ECVec};

pub struct State{
    pub g_man:GenManager,
    pub positions:ECVec<Position>,
    pub tiles:ECVec<Tile>,
}

impl State{
    pub fn new()->Self{
        State{
            g_man:GenManager::new(),
            positions:ECVec::new(),
            tiles:ECVec::new(),
        }
    }

    pub fn add_tile(&mut self,t:Tile,p:Position){
        let gi = self.g_man.add();
        self.positions.put(gi,p);
        self.tiles.put(gi,t);
    }
}

pub struct Position{
    pub x:u32,
    pub y:u32,
}

impl Position{
    pub fn new(x:u32,y:u32)->Self{
        Position{x,y}
    }
}

pub enum Tile{
    Man,
    Block,
    Door(u8),
}

pub enum Wall{
    Clear,
    Wall,
    Spike,
}

pub enum DrawMode{
    No,
    Shape,
}

