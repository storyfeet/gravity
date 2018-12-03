use crate::ecs::gen::{GenManager,GenItem};
use crate::ecs::ec_vec::{ECVec};

use super::state::{State,Tile,PlayMode,GravCp};
use super::rects::{Position,UP};
use super::grid::EdgeGrid;

pub struct LevelSave{
    gens:GenManager,
    tiles:ECVec<Tile>,
    positions:ECVec<Position>,
    gravs:ECVec<GravCp>,
    walls:EdgeGrid,
}

pub fn save_level(s:&mut State){
    let mut gens = s.g_man.clone();
    let mut tiles = s.tiles.clone();
    let mut positions = s.grid_pos.clone();
    let mut walls = s.walls.clone();
    let mut gravs = s.gravs.clone();

    let cv = gens.compress();
    tiles.compress(&cv);
    positions.compress(&cv);
    gravs.compress(&cv);
    s.last_save = Some(LevelSave{gens,tiles,positions,walls,gravs});
}

pub fn restore_level(s:&mut State){
    if let Some(sv) = &mut s.last_save {
        s.walls = sv.walls.clone();
        s.g_man = sv.gens.clone();
        s.grid_pos = sv.positions.clone();
        s.tiles = sv.tiles.clone();
        s.gravs = sv.gravs.clone();
        s.draw = ECVec::new();

        s.gravity = UP;
        s.p_mode = PlayMode::Wait;
    }
}


