use crate::ecs::{GenManager,ECVec,GenItem};
use piston_window::rectangle::{Rectangle,Border};
use piston_window::ButtonState;
use crate::grid::EdgeGrid;
use crate::texture_loader::TexLoader;
use crate::rects::{UP,Position};




pub struct State{
    //Gen Controlled
    pub g_man:GenManager,
    pub grid_pos:ECVec<Position>,
    pub tiles:ECVec<Tile>,
    pub draw:ECVec<DrawCp>,
    pub gravs:ECVec<GravCp>,


    //Useful Data
    pub walls:EdgeGrid,
    pub gravity:usize,
    pub grav_processing:bool,
    pub d_time:f64,
    pub btn_ctrl:ButtonState,
    pub btn_shift:ButtonState,

    pub tex_map:TexLoader,
}

impl State{
    pub fn new()->Self{
        State{
            //Gen Controlled
            g_man:GenManager::new(),
            grid_pos:ECVec::new(),
            tiles:ECVec::new(),
            draw:ECVec::new(), 
            gravs:ECVec::new(),


            //useful Data
            walls:EdgeGrid::new(3,3),
            gravity:UP,
            grav_processing:false,
            d_time:0.0,
            btn_ctrl:ButtonState::Release,
            btn_shift:ButtonState::Release,

            tex_map:TexLoader::new(),
        }
    }

    pub fn add_tile(&mut self,t:Tile,p:Position)->GenItem{
        let gi = self.g_man.add();
        self.grid_pos.put(gi,p);
        self.tiles.put(gi,t);
        gi
    }

    pub fn drop(&mut self,gi:GenItem){
        self.g_man.drop_item(gi);
        self.grid_pos.drop(gi);
        self.tiles.drop(gi);
        self.draw.drop(gi);
        self.gravs.drop(gi);
    }
}


#[derive(PartialEq,Debug)]
pub enum Tile{
    Editor,
    Man,
    Block,
}


pub struct DrawCp{
    pub z:u8,
    pub rect:[f64;4],
    pub mode:DrawMode,
}

pub enum DrawMode{
    Rect(Rectangle),//pist_rect
	Tex(usize,usize)//tex_ref, angle
}

#[derive(Copy,Clone)]
pub struct GravCp{
    pub priority:usize,
}
