use crate::state::{State,PlayMode,MoveAction,Tile};
use crate::ecs::GenItem;
use crate::rects::{Position,LEFT,RIGHT};
use crate::error::GravError;


pub fn timer_sys(s:&mut State,d:f64)->bool{
    s.d_time += d; 
    if s.d_time > 0.2 {
        s.d_time = 0.;
        return true;
    }
    false
}


fn move_in_dir(s:&mut State,mut ms:Vec<GenItem>,dir:usize)->Result<(),GravError>{
    let step = Position::from_dir(dir);
    println!("dir = {}, step = {:?}",dir,step);
    loop{
        let mut moved = false;
        let mut ms2 = Vec::new();
        for g in ms{
            let pos = *s.grid_pos.get(g).ok_or("no gridpos")?;
            let npos = pos + step;
            let mut b_found = false;
            for (at_gi,at_p)in s.grid_pos.iter(){
                if *at_p == npos {
                    match s.tiles.get(at_gi){
                        Some(Tile::Man)=>{
                            ms2.push(g);
                            b_found = true;
                        }
                        Some(Tile::Block)=>{
                            ms2.push(at_gi);
                            ms2.push(g);
                            b_found = true;
                        },
                        _=>continue,
                    }
                }
            }
            if !b_found{
                s.grid_pos.put(g,npos);
                moved = true;
            }
        }
        if !moved{
            return Ok(());
        }
        ms = ms2;
    }

}

pub fn move_sys(s:&mut State){
    
    let m_action = match s.p_mode {
        PlayMode::Move(ma)=>ma,
        _=>return,
    };

    // println!("PlayMode = {:?}",s.p_mode);
    let movers:Vec<GenItem> = s.tiles.iter().filter(|(_,t)| **t ==Tile::Man).map(|(g,_)|g).collect();
    match m_action{
        MoveAction::Lf=>{
            let dir = LEFT + s.gravity;
            move_in_dir(s,movers,dir);
        }
        MoveAction::Rt=>{
            let dir = RIGHT + s.gravity;
            move_in_dir(s,movers,dir);
        }
        _=>{},
    }
    s.p_mode = PlayMode::Grav;
}
