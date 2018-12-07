use piston_window::{Context,G2d,rectangle,text,Glyphs,Transformed};

use super::state::MenuState;


pub fn draw_sys(st:&mut MenuState,font:&mut Glyphs,c:Context,g:&mut G2d){

    let t = text::Text::new(30);
    for (gi,b) in &st.buttons{
        let sx = (gi.loc as f64 )*100.;
    rectangle([0.5, 0.5, 0.5, 1.0], //grey
                      [sx, 0.0, 80.0, 80.0], // rectangle
                      c.transform, g);
    t.draw(*b,font,&c.draw_state,c.transform.trans(sx,50.),g);
    }




    
}


