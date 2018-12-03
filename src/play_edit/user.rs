use piston_window::{ButtonArgs,keyboard::Key,Button,ButtonState};
use std::ops::{Sub,SubAssign};

use crate::ecs::{GenItem};
use crate::error::GravError;

use super::state::{State,Tile,GravCp,PlayMode,MoveAction};
use super::grid::{EdgeGrid};
use super::rects::{Position,UP,DOWN,LEFT,RIGHT};
use super::saver::{save_level,restore_level};

use self::Button::*;

fn toggle_tile(s:&mut State,tp:Position,skip:GenItem){
    let mut found:Option<GenItem> = None;
    for (gi,p) in s.grid_pos.iter() {
        if gi == skip{ continue} 
        if *p == tp {
            found = Some(gi);
            break;
        }
    }
    match found{ 
        None=>{
            let gi = s.add_tile(Tile::Man,tp);
            s.gravs.put(gi,GravCp{priority:1});
        },
        Some(gi)=>{
            match s.tiles.get(gi) {
                Some(Tile::Man)=>s.tiles.put(gi,Tile::Block),//TODO consider gravity
                _=>s.drop(gi),
            };
        },
    };
}


fn _dec<T:PartialOrd +Sub<Output=T>+Copy>(v:T,n:T)->T{
    return if v < n{
        v - v
    }else {
        v-n
    };
}

fn _dec_ip<T:PartialOrd + SubAssign+Copy>(v:&mut T,n:T){
    if *v < n {
        *v -= *v;
        return;
    }
    *v -= n;
}


pub fn key_sys(s:&mut State,k:ButtonArgs)->Result<(),GravError>{
    if k.state != ButtonState::Press{
        match k.button {
            Keyboard(Key::LCtrl)=>s.btn_ctrl = ButtonState::Release,
            Keyboard(Key::LShift)=>s.btn_shift = ButtonState::Release,
            _=>{},
        }
        return Ok(())
    }
    match k.button{
        Keyboard(Key::LCtrl)=>s.btn_ctrl = ButtonState::Press,
        Keyboard(Key::LShift)=>s.btn_shift = ButtonState::Press,
        _=>{},
    }
    
    if s.btn_shift == ButtonState::Press && s.btn_ctrl == ButtonState::Press{
        let (sw,sh) = (s.walls.w,s.walls.h);
        match k.button {
            Keyboard(Key::Left)=>s.walls = EdgeGrid::new(_dec(sw,1),sh),
            Keyboard(Key::Right)=>s.walls = EdgeGrid::new(sw+1,sh),
            Keyboard(Key::Up)=>s.walls = EdgeGrid::new(sw,_dec(sh,1)),
            Keyboard(Key::Down)=>s.walls = EdgeGrid::new(sw,sh+1),
            _=>{},
        }
        return Ok(())
    }

    //Stuff that doesn't need an editor
    match k.button {
        Keyboard(Key::E)=>{
            if let Some((ed_ref,_)) = s.tiles.iter().find(|(_,t)|**t == Tile::Editor){
                s.drop(ed_ref);
                return Ok(());
            }
            s.add_tile(Tile::Editor,Position{x:0,y:0});
            return Ok(());
        }
        Keyboard(Key::H)=> s.p_mode= PlayMode::Grav,
        Keyboard(Key::S)=> save_level(s),
        Keyboard(Key::R)=> restore_level(s),

        
        _=>{},
    }


    //If Editor Exists
    if let Some((ed_ref,_)) = s.tiles.iter().find(|(_,t)|**t == Tile::Editor){
        let ed_pos = *s.grid_pos.get(ed_ref).unwrap_or(&Position::new(0,0));

        if s.btn_ctrl == ButtonState::Press{
            match k.button {
                Keyboard(Key::Left)=>s.walls.toggle_edge(ed_pos,LEFT)?,
                Keyboard(Key::Right)=>s.walls.toggle_edge(ed_pos,RIGHT)?,
                Keyboard(Key::Up)=>s.walls.toggle_edge(ed_pos,UP)?,
                Keyboard(Key::Down)=>s.walls.toggle_edge(ed_pos,DOWN)?,
                _=>{},
            }
            return Ok(())
        }
        match k.button{
            Keyboard(Key::Left)=> s.grid_pos.put(ed_ref,ed_pos+Position::new(-1,0)),
            Keyboard(Key::Right)=> s.grid_pos.put(ed_ref,ed_pos+Position::new(1,0)),
            Keyboard(Key::Up)=> s.grid_pos.put(ed_ref,ed_pos+Position::new(0,-1)),
            Keyboard(Key::Down)=> s.grid_pos.put(ed_ref,ed_pos+Position::new(0,1)),
            Keyboard(Key::Space)=> toggle_tile(s,ed_pos,ed_ref),
            Keyboard(Key::G)=> {s.walls.toggle_core(ed_pos);},
            
            _=>{},
        }
        return Ok(());
    }

    //Normal Movement
    if s.p_mode != PlayMode::Wait {
        return Ok(());
    }
    match k.button {
        Keyboard(Key::Left)=>s.p_mode = PlayMode::Move( 
            if s.btn_ctrl == ButtonState::Press{ 
                MoveAction::LfFar
            }else if s.btn_shift == ButtonState::Press{
                MoveAction::LfUp
            }else {
                MoveAction::Lf
            }
        ),        
        Keyboard(Key::Right)=>s.p_mode = PlayMode::Move( 
            if s.btn_ctrl == ButtonState::Press{ 
                MoveAction::RtFar
            }else if s.btn_shift == ButtonState::Press{
                MoveAction::RtUp
            }else {
                MoveAction::Rt
            }
        ),        
        Keyboard(Key::Up)=>s.p_mode = PlayMode::Move(MoveAction::Jmp),
        _=>{},

    }
    Ok(())
}
