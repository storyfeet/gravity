use piston_window::{PistonWindow,WindowSettings,Event,Loop,clear,Input};

mod ecs;
mod texture_loader;
mod error;
mod play_edit;

fn main() {

    let mut g_state = play_edit::state::State::new();


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
    g_state.tex_map.load(&mut window.factory,"assets/arrow.png").unwrap();
    println!("Loaded");

    while let Some(e) = window.next(){        
        self::play_edit::as_scene(&mut window,e,&mut g_state) ;
    }
}
