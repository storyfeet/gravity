use crate::ecs::{GenManager,ECVec,GenItem};
use piston_window::rectangle::{Rectangle,Border};


pub struct State{
    //Gen Controlled
    pub g_man:GenManager,
    pub grid_pos:ECVec<Position>,
    pub tiles:ECVec<Tile>,
    pub draw:ECVec<DrawCp>,

    //Indexes
    pub ls_tiles:Vec<GenItem>,
    pub ls_draw:Vec<GenItem>,

    //Useful Data
    pub d_time:f64,
    pub p_ref:GenItem,
}

impl State{
    pub fn new()->Self{
        let mut g_man = GenManager::new();
        let p_ref = g_man.add();
        let mut res = State{
            //Gen Controlled
            g_man,
            grid_pos:ECVec::new(),
            tiles:ECVec::new(),
            draw:ECVec::new(), 

            //Indexes
            ls_tiles:Vec::new(),
            ls_draw:Vec::new(),

            //useful Data
            d_time:0.0,
            p_ref,
        };
        res.grid_pos.put(p_ref,Position{x:0,y:0});
        res.tiles.put(p_ref,Tile::Editor);
        res.ls_tiles.push(p_ref);
        res
    }

    pub fn add_tile(&mut self,t:Tile,p:Position){
        let gi = self.g_man.add();
        self.grid_pos.put(gi,p);
        self.tiles.put(gi,t);
        self.ls_tiles.push(gi);
    }

    pub fn drop(&mut self,gi:GenItem){
        self.g_man.drop_item(gi);
        self.grid_pos.drop(gi);
        self.tiles.drop(gi);
        self.draw.drop(gi);

        //Indexes
        self.ls_tiles.retain(|&x| x!= gi);
        self.ls_draw.retain(|&x| x!= gi);


    }
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub struct Position{
    pub x:u8,
    pub y:u8,
}

impl Position{
    pub fn new(x:u8,y:u8)->Self{
        Position{x,y}
    }
}

pub enum Tile{
    Editor,
    Man,
    Block,
    Door(u8),
}

pub enum Wall{
    Clear,
    Wall,
    Spike,
}

pub struct DrawCp{
    pub r:[f64;4],//position x,y,w,h
    pub z:u8,
    pub mode:DrawMode,
}

pub enum DrawMode{
    Rect([f32;4]),
}

