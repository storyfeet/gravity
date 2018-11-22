use crate::state::{State,Position,Tile,DrawMode,DrawCp};
use piston_window::rectangle::{Rectangle,Border};

use piston_window::{Context,G2d,draw_state,clear};

pub fn tile_to_draw_sys(s:&mut State)->Option<()>{
    for gi in &s.ls_tiles{
        let Position{x,y} = s.grid_pos.get(*gi)?;
        let r = [(*x as f64) * 50.,*y as f64 * 50.,50.,50.];
        let mode = match s.tiles.get(*gi)?{
            Tile::Man=>DrawMode::Rect([0.,1.,1.,1.]),
            Tile::Block=>DrawMode::Rect([0.,0.,1.,1.]),
            Tile::Door(_)=>DrawMode::Rect([0.5,0.5,0.5,1.]),
        };
        s.draw.put(*gi,DrawCp{r,mode});
    }
    Some(())
}

pub fn draw_sys(s:&State,c:Context,g:&mut G2d){
    clear([1.,0.,0.,1.],g);
    let border = Border{color:[0.,0.,0.,1.],radius:2.};
    for (gi,pi) in s.draw.iter(){
        match pi.mode {
            DrawMode::Rect(col)=> Rectangle::new(col)
                        .border(border)
                        .draw(pi.r, &draw_state::DrawState::new_alpha(),
                              c.transform,g),
        }
    }
}

