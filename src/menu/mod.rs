use piston_window::{Window,PistonWindow,Event,Loop,clear,Input,Glyphs};
use anymap::AnyMap;
use std::cell::RefCell;

use crate::scene::SceneAction;
use crate::error::GravError;
use self::state::MenuState;


mod draw;
mod state;
mod input;


pub fn setup(folder:String)->MenuState{
    let mut n_state = MenuState::new(folder);
    n_state.add_button("Play");
    n_state.add_button("Edit");
    n_state.add_button("Load");
    let save_id = n_state.add_button("Save");
    n_state.texts.put(save_id,String::new());
    n_state.add_button("Quit");

    n_state
}

pub fn as_scene(window:&mut PistonWindow,e:Event,state_map:&mut AnyMap)->Result<SceneAction,GravError>{

    let m_state = state_map.get::<RefCell<MenuState>>()
              .ok_or("MenuState not set")?;
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
        Event::Input(Input::Text(s))=>{
            println!("TEXT:{}",s);
        }
        _=>{},
    }

    Ok(SceneAction::Cont)
}




