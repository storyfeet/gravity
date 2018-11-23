use piston_window::{ButtonArgs,keyboard::Key,Button,ButtonState};
use crate::state::{State,Tile,Position};
use crate::ecs::{GenItem};
use self::Button::*;

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
