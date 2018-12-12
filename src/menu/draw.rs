use piston_window::{Context,G2d,rectangle,text,Glyphs,Transformed,Size};

use super::state::MenuState;

const GRAY:[f32;4] = [0.5,0.5,0.5,1.];
const L_GRAY:[f32;4] = [0.8,0.8,0.8,1.];


pub fn draw_sys(st:&mut MenuState,font:&mut Glyphs,sz:Size,c:Context,g:&mut G2d){

    let t = text::Text::new(30);
    for (gi,b) in &st.buttons{
        let sy = (gi.loc as f64 )*50. + 100.;
        let col = if Some(gi)== st.focus { L_GRAY} else {GRAY};
    rectangle(col, [sz.width*0.25, sy,sz.width*0.5, 40.0], // rectangle
                      c.transform, g);
    t.draw(*b,font,&c.draw_state,c.transform.trans(sz.width*0.5-(b.len() as f64)*12.
                                                   ,sy+34.),g);
    }

}
