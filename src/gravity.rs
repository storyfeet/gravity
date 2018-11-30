use crate::state::{State,GravCp,PlayMode};
use crate::rects::{dir_as_pos};
use crate::ecs::gen::GenItem;
use crate::grid::{Edge,can_pass};


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

