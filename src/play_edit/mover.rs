use crate::ecs::GenItem;
use crate::error::GravError;

use super::grid::can_pass;
use super::state::{State,PlayMode,MoveAction,Tile,AnimState};
use super::rects::{Position,LEFT,RIGHT,UP};



pub fn timer_sys(s:&mut State,d:f64)->bool{
    s.d_time += d; 
    if s.d_time > 0.1 {
        s.d_time = 0.;
        return true;
    }
    false
}


fn move_in_dir(s:&mut State,mut ms:Vec<GenItem>,dir:usize,anim:AnimState)->bool{
    let step = Position::from_dir(dir);
    //println!("dir = {}, step = {:?}",dir,step);
    let mut res = false;
    loop{
        let mut moved = false;
        let mut ms2 = Vec::new();
        for g in ms{
            let pos = match s.grid_pos.get(g){
                Some(s)=>*s,
                _=>continue,
            };

            let npos = pos + step;
            if ! can_pass(s.walls.edge_at(pos,dir)){continue}
            if ! can_pass(s.walls.edge_at(npos,dir+2)){continue}

            let mut b_found = false;
            let bumps = s.grid_pos.iter()
                        .filter(|(at_gi,at_p)|{
                            return if **at_p ==npos{
                                match s.tiles.get(*at_gi){
                                    Some(Tile::Man)=>{
                                        b_found = true;
                                        false
                                    },
                                    Some(Tile::Block)=>{
                                        b_found = true;
                                        true
                                    },
                                    _=>false
                                }
                            } else {false};
                        })
                        .map(|(g,_)|g).collect();

            moved = move_in_dir(s,bumps,dir,anim) ||moved;
            if b_found{
                ms2.push(g);
            }else{
                s.grid_pos.put(g,npos);
                if let Some(Tile::Man) = s.tiles.get(g){
                    s.anims.put(g,anim);
                }
                moved = true;
            }
        }
        if !moved{
            return res;
        }
        res = true;
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
            move_in_dir(s,movers,dir,AnimState::Lf);
            s.p_mode = PlayMode::Grav;
        }
        MoveAction::Rt=>{
            let dir = RIGHT + s.gravity;
            move_in_dir(s,movers,dir,AnimState::Rt);
            s.p_mode = PlayMode::Grav;
        }
        MoveAction::Jmp=>{
            let dir = UP + s.gravity;
            move_in_dir(s,movers,dir,AnimState::Jmp);
            s.p_mode = PlayMode::Grav;
        }
        MoveAction::LfUp=>{
            let dir = UP + s.gravity;
            move_in_dir(s,movers,dir,AnimState::LfJmp);
            s.p_mode = PlayMode::Move(MoveAction::Lf);
        }
        MoveAction::RtUp=>{
            let dir = UP + s.gravity;
            move_in_dir(s,movers,dir,AnimState::RtJmp);
            s.p_mode = PlayMode::Move(MoveAction::Rt);
        }
        MoveAction::LfFar=>{
            let dir = LEFT + s.gravity;
            move_in_dir(s,movers,dir,AnimState::LfJmp);
            s.p_mode = PlayMode::Move(MoveAction::Lf);
        }
        MoveAction::RtFar=>{
            let dir = RIGHT + s.gravity;
            move_in_dir(s,movers,dir,AnimState::RtJmp);
            s.p_mode = PlayMode::Move(MoveAction::Rt);
        }
    }
}

