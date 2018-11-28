use piston_window::{PistonWindow,WindowSettings,Event,Loop,clear,draw_state,Input};

mod ecs;
mod state;
mod draw;
//mod mover;
mod user;
mod grid;
mod rects;
mod texture_loader;
mod gravity;
mod error;

fn main() {

    let mut g_state = state::State::new();


    let mut window:PistonWindow = 
        WindowSettings::new("Gravity",[640,480])
                    .exit_on_esc(true)
                    .build()
                    .unwrap();



    while let Some(e) = window.next(){        

        window.draw_2d(&e,|c,g|{
            clear([1.,0.,0.,1.],g);
            draw::grid_draw_sys(&g_state,c,g);
            draw::draw_sys(&mut g_state,c,g);
        });
        match e {
            Event::Input(Input::Button(bargs))=>{
                user::key_sys(&mut g_state,bargs);
            }
            
            Event::Loop(Loop::Update(d))=>{
                //mover::move_sys(&mut g_state,d.dt);
                gravity::gravity_sys(&mut g_state);
                draw::tile_to_draw_sys(&mut g_state,&mut window.factory);
            },
            _=>{},//println!("OTHER {:?}",e),
        }

    }
}
