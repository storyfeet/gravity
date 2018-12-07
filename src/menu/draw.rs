use piston_window::{Context,G2d,rectangle};


pub fn draw_sys(c:Context,g:&mut G2d){
    rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform, g);
    
}


