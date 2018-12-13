use piston_window::{Context,G2d,rectangle,text,Glyphs,Transformed,Size};

use super::state::MenuState;

const GRAY:[f32;4] = [0.5,0.5,0.5,1.];
const L_GRAY:[f32;4] = [0.8,0.8,0.8,1.];
const WHITE:[f32;4] = [1.,1.,1.,1.];


pub fn draw_sys(st:&mut MenuState,font:&mut Glyphs,sz:Size,c:Context,g:&mut G2d){

    let t = text::Text::new(30);
    let b_width = sz.width * 0.4;
    for (gi,b) in &st.buttons{
        let sy = (gi.loc as f64 )*50. + 100.;
        let col = if Some(gi)== st.focus { L_GRAY} else {GRAY};
        let trans = if let Some(tx) = st.texts.get(gi){
            rectangle(WHITE,[sz.width*0.5, sy+5.,sz.width*0.4,30.],c.transform,g);
            t.draw(&tx,font,&c.draw_state,c.transform.trans(sz.width*0.5,sy+30.),g).ok();

            c.transform.trans(sz.width*0.1,sy)

        }else {c.transform.trans(sz.width*0.3,sy)};
        rectangle(col,[0.,0.,b_width,40.], trans, g);
        t.draw(*b,font,&c.draw_state,
                trans.trans(b_width*0.5-(b.len() as f64)*12.,34.),
                g).ok();
    }

}
