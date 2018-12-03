use piston_window::rectangle::{Rectangle};
use piston_window::ButtonState;

use crate::ecs::{GenManager,ECVec,GenItem};
use crate::texture_loader::TexLoader;

use super::grid::EdgeGrid;
use super::rects::{UP,Position};
use super::saver::LevelSave;




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
    pub p_mode:PlayMode,
    pub d_time:f64,
    pub btn_ctrl:ButtonState,
    pub btn_shift:ButtonState,

    pub tex_map:TexLoader,

    pub last_save:Option<LevelSave>,
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
            walls:EdgeGrid::new(10,8),
            gravity:UP,
            p_mode:PlayMode::Wait,
            d_time:0.0,
            btn_ctrl:ButtonState::Release,
            btn_shift:ButtonState::Release,

            tex_map:TexLoader::new(),
            last_save:None,
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


#[derive(Copy,Clone,PartialEq,Debug)]
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

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum PlayMode{
    Wait,
    Grav,
    Move(MoveAction), 
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum MoveAction{
    Lf,Rt,Jmp,LfUp,RtUp,LfFar,RtFar
}


