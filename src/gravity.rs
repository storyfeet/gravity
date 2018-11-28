use crate::state::{State,GravCp};
use crate::rects::{dir_as_pos};
use crate::ecs::gen::GenItem;


pub fn gravity_sys(s:&mut State){
    if ! s.gravity_processing{return}
    let mut v:Vec<(GenItem,GravCp)>;
    
    for (gi,gc) in s.gravs.iter(){ 
        //check for wall, then add to movers;
          
        

        v.push((gi,gc));
    }
    let mut moved = true;
    while moved {
        moved = false;

    }
}

