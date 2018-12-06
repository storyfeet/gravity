use piston_window::{PistonWindow,WindowSettings,Event,Loop,clear,Input};
use anymap::AnyMap;
use std::cell::RefCell;
use self::draw;

pub fn as_scene(window:&mut PistonWindow,e:Event,state_map:&mut AnyMap){
    window.draw_2d(&e,|c,g|{
        clear([1.,1.,0.,1.],g);
        draw::draw_sys(c,g);
    });
}




