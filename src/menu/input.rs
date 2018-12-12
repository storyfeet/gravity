use piston_window::{ButtonArgs,keyboard::Key,Button::Keyboard,ButtonState};

use crate::error::{GravError,GravError::NonError};
use crate::scene::SceneAction;

use super::state::MenuState;





pub fn key_sys(st:&mut MenuState,ba:ButtonArgs)->Result<SceneAction,GravError>{
      
    if ba.state == ButtonState::Press {
        match ba.button {
            Keyboard(Key::Down)=>{
                match st.focus{
                    Some(curr)=>{
                    let mut found = false;
                        for (gi,_) in &st.buttons {
                            if found {
                                st.focus = Some(gi);
                                return Ok(SceneAction::Cont);
                            }
                            if gi == curr{
                                found = true;
                            }
                        }
                    },
                    None=>st.focus = st.buttons.iter().next().map(|(g,_)|g),
                }
            }
            Keyboard(Key::Up)=>{
                match st.focus{
                    Some(curr)=>{
                        let mut last = None;
                        for (gi,_) in &st.buttons{
                            if gi == curr{
                                st.focus = last; 
                            }
                            last = Some(gi);
                        }
                    },
                    None=>st.focus = st.buttons.iter().next().map(|(g,_)|g),
                }
            },
            Keyboard(Key::Escape)=>return Ok(SceneAction::DropOff),
            Keyboard(Key::Return)=>{ 
                if let Some(f) = st.focus{
                    match st.buttons.get(f).map(|s|*s){
                        Some("Play")=>return Ok(SceneAction::Child("PLAY")),
                        Some("Quit")=>return Ok(SceneAction::DropOff),
                        Some("Save")=>{
                            //let r = nfd::dialog_save().open();
                            println!("save to {:?}","--Somewhere--");
                        },
                        s=>{println!("Return - {:?}",s);},
                        
                    }
                }
            },
            _=>{},
        }
    }

    Ok(SceneAction::Cont)
}

pub fn text_sys(st:&mut MenuState,s:String)->Result<(),GravError>{
    let f = st.focus.ok_or(NonError)?;

    let t = st.texts.get_mut(f).ok_or(NonError)?;
    Ok(())
}
