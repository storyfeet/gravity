use piston_window::{PistonWindow,WindowSettings,Event,Loop,clear,Input};

pub mod state;
use self::state::State;
pub mod draw;
pub mod mover;
pub mod user;
pub mod grid;
pub mod rects;
pub mod gravity;
pub mod saver;


pub fn as_scene(window:&mut PistonWindow,e:Event,g_state:&mut State){
    window.draw_2d(&e,|c,g|{
        clear([1.,0.,0.,1.],g);
        draw::grid_draw_sys(g_state,c,g);
        draw::draw_sys(g_state,c,g);
    });
    match e {
        Event::Input(Input::Button(bargs))=>{
            user::key_sys(g_state,bargs);
        }
        
        Event::Loop(Loop::Update(d))=>{
            if mover::timer_sys(g_state,d.dt) {
                gravity::gravity_sys(g_state);
                mover::move_sys(g_state);
                gravity::gravity_arrow_sys(g_state);
            }
            draw::tile_to_draw_sys(g_state);
        },
        _=>{},//println!("OTHER {:?}",e),
    }


}
