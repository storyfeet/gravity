
use std::cell::RefCell;
use piston_window::{PistonWindow,WindowSettings,Glyphs,TextureSettings};

use lazy_conf::config;
use crate::scene::SceneAction;


mod ecs;
mod texture_loader;
mod error;
mod play_edit;
mod scene;
mod menu;
mod svg;

pub enum SceneSelection{
    Menu,
    Play,
    Edit,
}
use self::SceneSelection::*;


fn main(){

    let mut conf = config("-c",&["test_data/conf.lz","{HOME}/.config/gravity/conf.lz"]).expect("config file path badly formed");
    
    let l_folder = conf.grab().cf("config.folder").fg("-f")
                        .help("Folder for Levels")
                        .s().unwrap_or("levels".to_string());

    
    let mut state_map = anymap::AnyMap::new();

    state_map.insert(RefCell::new(play_edit::state::State::new()));

    state_map.insert(RefCell::new(menu::setup(l_folder)));
    

    let mut window:PistonWindow = 
        WindowSettings::new("Gravity",[640,480])
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
    tex_map.load(&mut window.factory,"assets/man_tr/man_00.png").unwrap();
    tex_map.load(&mut window.factory,"assets/man_tr/man_01.png").unwrap();
    tex_map.load(&mut window.factory,"assets/man_tr/man_02.png").unwrap();
    tex_map.load(&mut window.factory,"assets/man_tr/man_03.png").unwrap();
    tex_map.load(&mut window.factory,"assets/man_tr/man_04.png").unwrap();
    tex_map.load(&mut window.factory,"assets/man_tr/man_05.png").unwrap();

    let font = Glyphs::new("assets/fonts/data-latin.ttf",window.factory.clone(),TextureSettings::new()).unwrap();
    println!("Loaded");

    state_map.insert(tex_map);
    state_map.insert(RefCell::new(font));

    let mut scene_stack = Vec::new();
    scene_stack.push(Menu);

    while let Some(e) = window.next(){        

        if scene_stack.len() == 0 {
            println!("Byee");
            return;
        }

        let s_res = match scene_stack[scene_stack.len()-1] {
            Menu=>self::menu::as_scene(&mut window,e,&mut state_map),
            _=>self::play_edit::as_scene(&mut window,e,&mut state_map),
        };
        
        match s_res{
            Ok(SceneAction::Quit)=>return,
            Ok(SceneAction::DropOff)=>{scene_stack.pop();},
            Ok(SceneAction::Child(s))=>match s{
                "PLAY"=>scene_stack.push(Play),
                _=>{},
            }
            Err(e)=>{println!("{:?}",e);},
            _=>{},
        }
    }
}


