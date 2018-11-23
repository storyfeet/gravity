use piston_window::{ButtonArgs,keyboard::Key,Button,ButtonState};
use crate::state::{State,Tile,Position};
use self::Button::*;

fn toggle_tile(s:&mut State,tp:Position){
    let gpos = &mut s.grid_pos;
    let gtile = &mut s.tiles;
    let p_ref = s.p_ref;
    let mut found = false;
    gpos.for_each(|gi,p|{
        if gi == p_ref{return}
        if tp == *p {
            if let Some(t) = gtile.get_mut(gi){
                *t = match t {
                    Tile::Man=>Tile::Door(1),
                    _=>Tile::Man,
                };
                found = true;
            }
        }
    });

    if !found {
        s.add_tile(Tile::Door(0),tp);
    }
}

pub fn key_sys(s:&mut State,k:ButtonArgs)->Option<()>{
    if k.state != ButtonState::Press{
        return None//consider Some here
    }
    match k.button{
        Keyboard(Key::Left)=> s.grid_pos.get_mut(s.p_ref)?.x -=1,
        Keyboard(Key::Right)=> s.grid_pos.get_mut(s.p_ref)?.x +=1,
        Keyboard(Key::Up)=> s.grid_pos.get_mut(s.p_ref)?.y -=1,
        Keyboard(Key::Down)=> s.grid_pos.get_mut(s.p_ref)?.y +=1,
        Keyboard(Key::Space)=> {
            let p = *s.grid_pos.get(s.p_ref)?;
            toggle_tile(s,p);
        }
        _=>{},
    }
    Some(())
}
