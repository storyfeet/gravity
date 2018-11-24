use piston_window::{ButtonArgs,keyboard::Key,Button,ButtonState};
use crate::state::{State,Tile,Position};
use crate::ecs::{GenItem};
use self::Button::*;
use crate::grid::{UP,DOWN,LEFT,RIGHT};

fn toggle_tile(s:&mut State,tp:Position)->Option<()>{
    let mut found:Option<GenItem> = None;
    for (gi,p) in s.grid_pos.iter() {
        if gi == s.p_ref{ continue} 
        if *p == tp {
            found = Some(gi);
            break;
        }
    }
    match found{ 
        None=>s.add_tile(Tile::Man,tp),
        Some(gi)=>{
            match *s.tiles.get(gi)? {
                Tile::Man=>*s.tiles.get_mut(gi)? = Tile::Door(1),
                _=>s.drop(gi),
            };
        },
    };
    Some(())
}

fn _dec(v:&mut u8,n:u8){
    if *v < n {
        *v = 0;
        return
    }
    *v -= n;
}


pub fn key_sys(s:&mut State,k:ButtonArgs)->Option<()>{
    if k.state != ButtonState::Press{
        match k.button {
            Keyboard(Key::LCtrl)=>s.ctrl_state = ButtonState::Release,
            _=>{},
        }
        return None//consider Some here
    }
    if s.ctrl_state == ButtonState::Press{
        match k.button {
            Keyboard(Key::Left)=>s.walls.toggle_wall(*s.grid_pos.get(s.p_ref)?,LEFT),
            Keyboard(Key::Right)=>s.walls.toggle_wall(*s.grid_pos.get(s.p_ref)?,RIGHT),
            Keyboard(Key::Up)=>s.walls.toggle_wall(*s.grid_pos.get(s.p_ref)?,UP),
            Keyboard(Key::Down)=>s.walls.toggle_wall(*s.grid_pos.get(s.p_ref)?,DOWN),
            _=>{},
        }
        return Some(())
    }
    match k.button{
        Keyboard(Key::LCtrl)=>s.ctrl_state = ButtonState::Press,
        Keyboard(Key::Left)=> _dec(&mut s.grid_pos.get_mut(s.p_ref)?.x ,1),
        Keyboard(Key::Right)=> s.grid_pos.get_mut(s.p_ref)?.x +=1,
        Keyboard(Key::Up)=> _dec(&mut s.grid_pos.get_mut(s.p_ref)?.y,1),
        Keyboard(Key::Down)=> s.grid_pos.get_mut(s.p_ref)?.y +=1,
        Keyboard(Key::Space)=> {
            let p = *s.grid_pos.get(s.p_ref)?;
            toggle_tile(s,p);
        }
        _=>{},
    }
    Some(())
}
