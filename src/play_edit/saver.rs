use serde_derive::{Deserialize, Serialize};

use crate::ecs::ec_vec::ECVec;
use crate::ecs::gen::GenManager;

use super::grid::EdgeGrid;
use super::rects::{Position, UP};
use super::state::{GravCp, PlayMode, State, Tile};

#[derive(Serialize, Deserialize)]
pub struct LevelSave {
    pub gens: GenManager,
    pub tiles: ECVec<Tile>,
    pub positions: ECVec<Position>,
    pub gravs: ECVec<GravCp>,
    pub walls: EdgeGrid,
}

pub fn save_level(s: &mut State) {
    if let Some(gi) = s
        .tiles
        .iter()
        .find(|(_, t)| **t == Tile::Editor)
        .map(|(gi, _)| gi)
    {
        s.drop(gi);
    }
    let mut gens = s.g_man.clone();
    let mut tiles = s.tiles.clone();
    let mut positions = s.grid_pos.clone();
    let mut gravs = s.gravs.clone();
    let walls = s.walls.clone();

    let cv = gens.compress();
    tiles.compress(&cv);
    positions.compress(&cv);
    gravs.compress(&cv);
    s.last_save = Some(LevelSave {
        gens,
        tiles,
        positions,
        walls,
        gravs,
    });
}

pub fn restore_level(s: &mut State) {
    if let Some(sv) = &mut s.last_save {
        s.walls = sv.walls.clone();
        s.g_man = sv.gens.clone();
        s.grid_pos = sv.positions.clone();
        s.tiles = sv.tiles.clone();
        s.gravs = sv.gravs.clone();

        s.gravity = UP;
        s.p_mode = PlayMode::Wait;
    }
}
