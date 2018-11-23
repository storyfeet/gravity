use crate::state::{State,Position,Tile,DrawMode,DrawCp};
use piston_window::rectangle::{Rectangle,Border};
use std::cmp::Ordering;

use piston_window::{Context,G2d,draw_state,clear};

pub fn tile_to_draw_sys(s:&mut State)->Option<()>{
    for gi in &s.ls_tiles{
        let Position{x,y} = s.grid_pos.get(*gi)?;
        let r = [(*x as f64) * 50.,*y as f64 * 50.,50.,50.];
        let (r,mode,z) = match s.tiles.get(*gi)?{
            Tile::Editor=>([r[0]+10.,r[1]+10.,r[2] - 20.,r[3]-20.],
                         DrawMode::Rect([1.,1.,1.,1.]),6),
            Tile::Man=>(r,DrawMode::Rect([0.,1.,1.,1.]),2),
            Tile::Block=>(r,DrawMode::Rect([0.,0.,1.,1.]),1),
            Tile::Door(_)=>(r,DrawMode::Rect([0.5,0.5,0.5,1.]),0)
        };
        s.draw.put(*gi,DrawCp{r,mode,z});
        if !s.ls_draw.contains(gi){
            s.ls_draw.push(*gi);
        }
    }
    Some(())
}

//For now the z sort happens here. I'd like to optimize to it only happens on a change, but not
//biggie
pub fn draw_sys(s:&mut State,c:Context,g:&mut G2d){
    let ls_draw = &mut s.ls_draw;
    let draw_list = &s.draw;
    ls_draw.sort_unstable_by(|ga,gb|{
        if let Some(a) = draw_list.get(*ga){
            if let Some(b) = draw_list.get(*gb){
                return a.z.cmp(&b.z);
            }
        }
        println!("Sort - Get Fail");
        return Ordering::Equal;
    });
    clear([1.,0.,0.,1.],g);
    let border = Border{color:[0.,0.,0.,1.],radius:2.};
    for gi  in ls_draw {
        if let Some(dc)=s.draw.get(*gi){
            match dc.mode{
                DrawMode::Rect(col)=> Rectangle::new(col)
                        .border(border)
                        .draw(dc.r, &draw_state::DrawState::new_alpha(),
                              c.transform,g),
            }
        }
    }
}

