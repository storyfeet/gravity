use piston_window::{PistonWindow,WindowSettings,Event,Loop,clear,Input};
use anymap::AnyMap;
use std::cell::RefCell;

use crate::texture_loader::TexLoader;
use crate::scene::SceneAction;
use crate::error::GravError;

pub mod state;
use self::state::State;
pub mod draw;
pub mod mover;
pub mod user;
pub mod grid;
pub mod rects;
pub mod gravity;
pub mod saver;


pub fn as_scene(window:&mut PistonWindow,e:Event,state_map:&mut AnyMap)->Result<SceneAction,GravError>{

    let mut g_state = state_map.get::<RefCell<State>>().unwrap().borrow_mut();

    let t_map:& TexLoader = state_map.get().unwrap();

    window.draw_2d(&e,|c,g|{
        clear([1.,0.,0.,1.],g);
        draw::grid_draw_sys(&g_state,t_map,c,g);
        draw::tile_draw_sys(&g_state,t_map,c,g);
    });
    match e {
        Event::Input(Input::Button(bargs))=>{
            match user::key_sys(&mut g_state,bargs)?{
                SceneAction::Cont=>{},
                r=>return Ok(r),
            }
        }
        
        Event::Loop(Loop::Update(d))=>{
            if mover::timer_sys(&mut g_state,d.dt) {
                gravity::gravity_sys(&mut g_state);
                mover::move_sys(&mut g_state);
                gravity::gravity_arrow_sys(&mut g_state);
            }
            //draw::tile_to_draw_sys(&mut g_state,t_map);
        },
        _=>{},//println!("OTHER {:?}",e),
    }

    Ok(SceneAction::Cont)

}
