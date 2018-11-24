use crate::state::{State,Position,Tile,DrawMode,DrawCp};
use crate::grid::{Wall,LEFT,UP,RIGHT,DOWN};
use piston_window::rectangle::{Rectangle,Border};
use piston_window::{Context,G2d,draw_state,line};
use std::cmp::Ordering;


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

pub fn _rot_about(x:f64,y:f64,cx:f64,cy:f64,ang:usize)->(f64,f64){
    //Assume UP to start;
    let (x,y) = (x-cx,y-cy);
    let (x,y) = match ang {
        LEFT=>(y,-x),
        DOWN=>(-x,-y),
        RIGHT=>(-y,x),
        _=>(x,y),
    };
    (x+cx,y+cy)
}

pub fn grid_draw_sys(s:&State,c:Context,g:&mut G2d){
    let border = Border{color:[0.,0.,0.,1.],radius:3.0};
    Rectangle::new([1.,1.,1.,1.])
            .border(border)
            .draw([0.,0.,s.walls.w as f64* 50., s.walls.h as f64*50.],
                    &draw_state::DrawState::new_alpha(),
                    c.transform,g);
    for (i,w) in s.walls.v.iter().enumerate() {
        let x = (i % s.walls.w) as f64;
        let y = (i / s.walls.w) as f64;
        let (x1,y1) = (x*50.,y*50.);
        let (x2,y2) = ((x+1.)*50.,y*50.);
        let (cx,cy) = ((x+0.5)*50.,(y+0.5)*50.);

        for dr in 0..4{
            let (dx1,dy1) = _rot_about(x1,y1,cx,cy,dr);
            let (dx2,dy2) = _rot_about(x2,y2,cx,cy,dr);

            match w[dr]{
                Wall::Line=>{
                    line([0.,0.,1.,1.],2.,[dx1,dy1,dx2,dy2],c.transform,g);
                }
                Wall::Spike=>{
                    line([1.,0.,0.,1.],2.,[dx1,dy1,dx2,dy2],c.transform,g);
                }
                _=>{},
            }
        }
    }

}
