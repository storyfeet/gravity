use crate::state::{State,Tile,DrawMode,DrawCp};
use crate::grid::{Edge};
use crate::ecs::gen::GenItem;
use crate::rects::{Position,UP,DOWN,LEFT,RIGHT,shrink_by,set_pos_angle,rot_about};

use piston_window::rectangle::{Rectangle,Border};
use piston_window::{Context,G2d,draw_state,line,Transformed};
use std::cmp::Ordering;
//use graphics::transformed::Transformed;

const COL_BAD:[f32;4]= [0.9,0.1,0.1,1.];


pub fn tile_to_draw_sys<F>(s:&mut State,fac:&mut F)->Option<()>
	where F: gfx_core::factory::Factory<gfx_device_gl::Resources>
{
    for (gi,t) in s.tiles.iter(){
        let Position{x,y} = s.grid_pos.get(gi)?;
        let r = [(*x as f64) * 50.,*y as f64 * 50.,50.,50.];
        let (z,rect,mode) = match s.tiles.get(gi)?{
            Tile::Editor=>{
				match s.tex_map.load(fac,"assets/cursor.png"){
					Ok(t_loc)=> (6,shrink_by(r,10.),DrawMode::Tex(t_loc,UP)),
					Err(_)=>(6,shrink_by(r,20.), DrawMode::Rect(Rectangle::new(COL_BAD))),
				}
			},
            Tile::Man=>
				match s.tex_map.load(fac,"assets/man.png"){
					Ok(t_loc)=> (5,r, DrawMode::Tex(t_loc,s.gravity)),
					Err(_)=>(5,r, DrawMode::Rect(Rectangle::new([1.,0.,0.,1.]))),
				}
            Tile::Block=>(4,r,DrawMode::Rect(Rectangle::new([0.,0.,1.,1.]))),
            //Tile::Door(_)=>(r,DrawMode::Rect([0.5,0.5,0.5,1.]),0)
        };
        s.draw.put(gi,DrawCp{z,mode,rect});
    }
    Some(())
}

pub fn draw_sys(s:&mut State,c:Context,g:&mut G2d){
    let mut ls_draw:Vec<GenItem> = s.draw.iter().map(|(g,_)|g.clone()).collect();

    ls_draw.sort_unstable_by(|ga,gb|{
        if let Some(a) = s.draw.get(*ga){
            if let Some(b) = s.draw.get(*gb){
                return a.z.cmp(&b.z);
            }
        }
        println!("Sort - Get Fail");
        return Ordering::Equal;
    });
    let border = Border{color:[0.,0.,0.,1.],radius:2.};

    for gi in ls_draw {
        if let Some(dc)=s.draw.get(gi){
            match dc.mode{
                DrawMode::Rect(rc)=> rc.draw(dc.rect, &draw_state::DrawState::new_alpha(),
                              c.transform,g),
				DrawMode::Tex(t_loc,t_ang)=>{
					if let Some(tx) = s.tex_map.get(t_loc){
						piston_window::image(tx,set_pos_angle(c.transform,dc.rect,t_ang),g);
					}
				}
            }
        }
    }
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
            let (dx1,dy1) = rot_about(x1,y1,cx,cy,dr);
            let (dx2,dy2) = rot_about(x2,y2,cx,cy,dr);

            match w[dr]{
                Edge::Wall=>{
                    line([0.,0.,1.,1.],2.,[dx1,dy1,dx2,dy2],c.transform,g);
                }
                Edge::Spike=>{
                    line([1.,0.,0.,1.],2.,[dx1,dy1,dx2,dy2],c.transform,g);
                }
                _=>{},
            }
        }
    }

}
