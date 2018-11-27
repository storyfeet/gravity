use crate::ecs::{GenManager,ECVec,GenItem};
use piston_window::rectangle::{Rectangle,Border};
use piston_window::ButtonState;
use crate::grid::EdgeGrid;
use crate::texture_loader::TexLoader;


pub struct State{
    //Gen Controlled
    pub g_man:GenManager,
    pub grid_pos:ECVec<Position>,
    pub tiles:ECVec<Tile>,
    pub draw:ECVec<DrawCp>,

    //Indexes
    pub ls_tiles:Vec<GenItem>,

    //Useful Data
    pub walls:EdgeGrid,
    pub d_time:f64,
    pub p_ref:GenItem,
    pub btn_ctrl:ButtonState,
    pub btn_shift:ButtonState,

    pub tex_map:TexLoader,
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

            //useful Data
            walls:EdgeGrid::new(3,3),
            d_time:0.0,
            p_ref,
            btn_ctrl:ButtonState::Release,
            btn_shift:ButtonState::Release,

            tex_map:TexLoader::new(),
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


pub struct DrawCp{
    pub r:[f64;4],//position x,y,w,h
    pub z:u8,
    pub mode:DrawMode,
}

pub enum DrawMode{
    Rect([f32;4]),
	Tex(usize),
}

