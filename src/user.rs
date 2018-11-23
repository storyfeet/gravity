use piston_window::{ButtonArgs,keyboard::Key,Button,ButtonState};
use crate::state::{State,Tile};
use self::Button::*;

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
            s.add_tile(Tile::Door(0),p);
        }
        _=>{},
    }
    Some(())
}
