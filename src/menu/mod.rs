use piston_window::{PistonWindow,WindowSettings,Event,Loop,clear,Input,keyboard::Key,Button::Keyboard,ButtonState};
use anymap::AnyMap;
use std::cell::RefCell;

use crate::scene::SceneAction;
use crate::error::GravError;
use self::state::MenuState;


mod draw;
mod state;

pub fn as_scene(window:&mut PistonWindow,e:Event,state_map:&mut AnyMap)->Result<SceneAction,GravError>{
    let m_state = state_map.get::<RefCell<MenuState>>();


    window.draw_2d(&e,|c,g|{
        clear([1.,1.,0.,1.],g);
        draw::draw_sys(c,g);
    });

    match e {
        Event::Input(Input::Button(bargs))=>{
            if bargs.state == ButtonState::Press {
                match bargs.button {
                    Keyboard(Key::Escape)=>return Ok(SceneAction::DropOff),
                    Keyboard(Key::Return)=>return Ok(SceneAction::Child("PLAY")),
                    _=>{},
                }
            }
        }
        _=>{},
    }

    Ok(SceneAction::Cont)
}




