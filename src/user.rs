use piston_window::{ButtonArgs,keyboard::Key,Button,ButtonState};
use crate::state::{State,Tile,Position};
use crate::ecs::{GenItem};
use self::Button::*;
use crate::grid::{EdgeGrid,UP,DOWN,LEFT,RIGHT};

use std::ops::{Sub,SubAssign};

fn toggle_tile(s:&mut State,tp:Position,skip:GenItem)->Option<()>{
    let mut found:Option<GenItem> = None;
    for (gi,p) in s.grid_pos.iter() {
        if gi == skip{ continue} 
        if *p == tp {
            found = Some(gi);
            break;
        }
    }
    match found{ 
        None=>s.add_tile(Tile::Man,tp),
        Some(gi)=>{
            match *s.tiles.get(gi)? {
                Tile::Man=>*s.tiles.get_mut(gi)? = Tile::Block,
                _=>s.drop(gi),
            };
        },
    };
    Some(())
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


pub fn key_sys(s:&mut State,k:ButtonArgs)->Option<()>{
    if k.state != ButtonState::Press{
        match k.button {
            Keyboard(Key::LCtrl)=>s.btn_ctrl = ButtonState::Release,
            Keyboard(Key::LShift)=>s.btn_shift = ButtonState::Release,
            _=>{},
        }
        return None//consider Some here
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
        return Some(())
    }

    //Stuff that doesn't need an editor
    match k.button {
        Keyboard(Key::E)=>{
            if let Some((ed_ref,_)) = s.tiles.iter().find(|(_,t)|**t == Tile::Editor){
                s.drop(ed_ref);
                return Some(());
            }
            s.add_tile(Tile::Editor,Position{x:0,y:0});
            return Some(());
        }
        Keyboard(Key::G)=>{
            s.gravity = (s.gravity + 1 )%4;
        }
        _=>{},
    }


    //If Editor Exists
    let (ed_ref,_) = s.tiles.iter().find(|(_,t)|**t == Tile::Editor)?;

    if s.btn_ctrl == ButtonState::Press{
        match k.button {
            Keyboard(Key::Left)=>s.walls.toggle_wall(*s.grid_pos.get(ed_ref)?,LEFT),
            Keyboard(Key::Right)=>s.walls.toggle_wall(*s.grid_pos.get(ed_ref)?,RIGHT),
            Keyboard(Key::Up)=>s.walls.toggle_wall(*s.grid_pos.get(ed_ref)?,UP),
            Keyboard(Key::Down)=>s.walls.toggle_wall(*s.grid_pos.get(ed_ref)?,DOWN),
            _=>{},
        }
        return Some(())
    }
    match k.button{
        Keyboard(Key::Left)=> _dec_ip(&mut s.grid_pos.get_mut(ed_ref)?.x ,1),
        Keyboard(Key::Right)=> s.grid_pos.get_mut(ed_ref)?.x +=1,
        Keyboard(Key::Up)=> _dec_ip(&mut s.grid_pos.get_mut(ed_ref)?.y,1),
        Keyboard(Key::Down)=> s.grid_pos.get_mut(ed_ref)?.y +=1,
        Keyboard(Key::Space)=> {
            let p = *s.grid_pos.get(ed_ref)?;
            toggle_tile(s,p,ed_ref);
        }
        _=>{},
    }
    Some(())
}
