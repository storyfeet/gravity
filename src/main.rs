use piston_window::{PistonWindow,WindowSettings,Event,Loop,clear,Input};
use std::cell::RefCell;

mod ecs;
mod texture_loader;
mod error;
mod play_edit;

fn main() {

    let mut state_map = anymap::AnyMap::new();

    state_map.insert(RefCell::new(play_edit::state::State::new()));
    

    let mut window:PistonWindow = 
        WindowSettings::new("Gravity",[640,480])
                    .exit_on_esc(true)
                    .samples(0)
                    .build()
                    .unwrap();

    println!("Loading");
    let mut tex_map = texture_loader::TexLoader::new();
    tex_map.load(&mut window.factory,"assets/door.png").unwrap();
    tex_map.load(&mut window.factory,"assets/man.png").unwrap();
    tex_map.load(&mut window.factory,"assets/cursor.png").unwrap();
    tex_map.load(&mut window.factory,"assets/spike.png").unwrap();
    tex_map.load(&mut window.factory,"assets/block.png").unwrap();
    tex_map.load(&mut window.factory,"assets/arrow.png").unwrap();
    tex_map.load(&mut window.factory,"assets/man_tr/man_01.png").unwrap();
    println!("Loaded");

    state_map.insert(tex_map);

    while let Some(e) = window.next(){        
        self::play_edit::as_scene(&mut window,e,&mut state_map) ;
    }
}
