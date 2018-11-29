use piston_window::{PistonWindow,WindowSettings,Event,Loop,clear,Input};

mod ecs;
mod state;
mod draw;
mod mover;
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

    println!("Loading");
    g_state.tex_map.load(&mut window.factory,"assets/door.png").unwrap();
    g_state.tex_map.load(&mut window.factory,"assets/man.png").unwrap();
    g_state.tex_map.load(&mut window.factory,"assets/cursor.png").unwrap();
    g_state.tex_map.load(&mut window.factory,"assets/spike.png").unwrap();
    g_state.tex_map.load(&mut window.factory,"assets/block.png").unwrap();
    println!("Loaded");

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
                if mover::timer_sys(&mut g_state,d.dt) {
                    gravity::gravity_sys(&mut g_state);
                    mover::move_sys(&mut g_state);
                }
                draw::tile_to_draw_sys(&mut g_state);
            },
            _=>{},//println!("OTHER {:?}",e),
        }

    }
}
