use crate::ecs::gen::GenItem;
use crate::error::GravError;

use super::state::{State,GravCp,PlayMode,Tile};
use super::rects::{dir_as_pos};
use super::grid::{Edge,TileCore,can_pass};


pub fn gravity_arrow_sys(s:&mut State)->Result<(),GravError>{
    
    let mut lowest_gen:Option<u64> = None;
    let mut ndir:Option<usize> = None;
    
    for (gi,t) in s.tiles.iter().filter(|(_,t)|**t == Tile::Man) {
        let p = s.grid_pos.get(gi).ok_or("No Position for Man Tile")?;
        if let Some(TileCore::GravChanger(dir)) = s.walls.core_at(*p){
            if let Some(n) = lowest_gen{
                if n < gi.gen {continue}
            }
            lowest_gen = Some(gi.gen);
            ndir = Some(dir);
        }
    }

    if let Some(nd) = ndir {
        if s.gravity != nd {
            s.gravity = nd;
            s.p_mode = PlayMode::Grav;
        }
    }
    Ok(())

}


pub fn gravity_sys(s:&mut State){
    if s.p_mode != PlayMode::Grav{return}

    s.p_mode = PlayMode::Wait;

    let mut v=Vec::new();
    
    for (gi,gc) in s.gravs.iter(){ 
        //check for wall, then add to movers;
        let pos = match s.grid_pos.get(gi){
            Some(pos)=>*pos,
            None=>continue,
        };
        let npos = pos + dir_as_pos(s.gravity +2);
        if can_pass(s.walls.edge_at(pos,s.gravity+2)) 
                && can_pass(s.walls.edge_at(npos,s.gravity)){
            v.push((gi,npos));
        }
          
    }
    let mut moved = true;
    while moved{ 
        moved = false;
        let mut v2 = Vec::new();
        for (gi,npos) in v{
            match s.grid_pos.iter().find(|(_,pos)|**pos == npos){
                Some(_)=>v2.push((gi,npos)),
                None=>{
                    s.grid_pos.put(gi,npos);
                    moved = true;
                    s.p_mode = PlayMode::Grav;
                },
            }
            
        }
        v = v2;
    }
}

