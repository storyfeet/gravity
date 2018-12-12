use piston_window::{Window,PistonWindow,Event,Loop,clear,Input,Glyphs};
use anymap::AnyMap;
use std::cell::RefCell;

use crate::scene::SceneAction;
use crate::error::GravError;
use self::state::MenuState;


mod draw;
mod state;
mod input;


pub fn as_scene(window:&mut PistonWindow,e:Event,state_map:&mut AnyMap)->Result<SceneAction,GravError>{
    let m_state = match state_map.get::<RefCell<MenuState>>(){
        Some(r)=>r,
        None=>{
            let mut n_state = MenuState::new();
            n_state.add_button("Play");
            n_state.add_button("Edit");
            n_state.add_button("Load");
            n_state.add_button("Save");
            n_state.add_button("Quit");

            state_map.insert(RefCell::new(n_state));
            state_map.get::<RefCell<MenuState>>().ok_or("Menu State not reachable")?

        }
    };
    let mut m_state = m_state.borrow_mut();

    let mut font=state_map.get::<RefCell<Glyphs>>().ok_or("Could not Get Font")?.borrow_mut();


    let sz = window.window.size();

    window.draw_2d(&e,|c,g|{
        clear([1.,1.,0.,1.],g);
        draw::draw_sys(&mut *m_state,&mut *font,sz,c,g);
    });

    match e {
        Event::Input(Input::Button(bargs))=>{
            return input::key_sys(&mut *m_state,bargs);
        }
        _=>{},
    }

    Ok(SceneAction::Cont)
}




