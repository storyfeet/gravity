use crate::state::{State,GravCp};
use crate::rects::{dir_as_pos};
use crate::ecs::gen::GenItem;
use crate::grid::Edge;


pub fn gravity_sys(s:&mut State){
    if ! s.grav_processing{return}
    let mut v=Vec::new();
    
    for (gi,gc) in s.gravs.iter(){ 
        //check for wall, then add to movers;
        let pos = match s.grid_pos.get(gi){
            Some(pos)=>pos,
            None=>continue,
        };
        if let Some(Edge::Clear) = s.walls.at(*pos,s.gravity){
            v.push((gi,*gc));
        }
          
    }
    let mut moved = true;
    while moved {
        moved = false;

    }
}

