use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

use piston_window::{ButtonArgs,keyboard::Key,Button::Keyboard,ButtonState};

use crate::error::{GravError,GravError::NonError};
use crate::scene::SceneAction;
use crate::play_edit::{state::State,saver::{LevelSave,restore_level}};

use super::state::MenuState;




pub fn key_sys(st:&mut MenuState,gst:&mut State,ba:ButtonArgs)->Result<SceneAction,GravError>{
      
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
            Keyboard(Key::Escape)=>match st.focus{
                Some(f)=>match st.texts.get(f){
                    Some(_)=>st.texts.drop(f),
                    None=>return Ok(SceneAction::DropOff),
                },
                None=>return Ok(SceneAction::DropOff),
            },
            Keyboard(Key::Return)=>{ 
                if let Some(f) = st.focus{
                    match st.buttons.get(f).map(|s|*s){
                        Some("Play")=>return Ok(SceneAction::Child("PLAY")),
                        Some("Quit")=>return Ok(SceneAction::DropOff),
                        Some("Save")=> match st.texts.get(f) {
                            Some(tx)=>{
                                let js = serde_json::to_string(&gst.last_save).expect("Could not Jsonify");
                                let path = PathBuf::from(&st.folder).join(tx);
                                let mut f = File::create(&path).map_err(|_|"Could not create file")?;
                                write!(f,"{}",js).map_err(|_|"could not write")?;

                                println!("{:?} - {}",path,js);
                              
                            },
                            None=>st.texts.put(f,"".to_string()),
                        },
                        Some("Load")=>match st.texts.get(f){
                            Some(tx)=>{
                                let path = PathBuf::from(&st.folder).join(tx);
                                let ld = File::open(path).map_err(|_|"Could not read")?;
                                let sv:LevelSave = serde_json::from_reader(ld).map_err(|_|"Could not jsonread")?;
                                gst.last_save=Some(sv);
                                
                                restore_level(gst);


                            },
                            None=>st.texts.put(f,"".to_string()),
                        },
                        s=>{println!("Return - {:?}",s);},
                        
                    }
                }
            },
            Keyboard(Key::Backspace)=>{
                if let Some(f) = st.focus{
                    if let Some(t) = st.texts.get_mut(f){
                        t.pop();
                    }
                }
            }
            _=>{},
        }
    }

    Ok(SceneAction::Cont)
}

pub fn text_sys(st:&mut MenuState,s:String)->Result<(),GravError>{
    let f = st.focus.ok_or(NonError)?;

    let t = st.texts.get_mut(f).ok_or(NonError)?;

    t.push_str(&s);
    Ok(())
}
