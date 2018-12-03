use piston_window::rectangle::{Rectangle,Border};
use piston_window::{image,Context,G2d,draw_state,line,Transformed};
use std::cmp::Ordering;

use crate::ecs::gen::GenItem;
use crate::error::GravError;
use crate::texture_loader::TexLoader;

use super::state::{State,Tile,DrawMode,DrawCp,AnimState};
use super::grid::{Edge,TileCore};
use super::rects::{Position,UP,DOWN,LEFT,RIGHT,shrink_by,set_pos_angle,rot_about};
//use graphics::transformed::Transformed;

const COL_BAD:[f32;4]= [0.9,0.1,0.1,1.];


pub fn tile_to_draw_sys(s:&mut State,tex_map:&TexLoader)->Result<(),GravError>{

    for (gi,t) in s.tiles.iter(){
        let Position{x,y} = s.grid_pos.get(gi).ok_or("Tile has no position")?;
        let r = [(*x as f64) * 50.,*y as f64 * 50.,50.,50.];
        let (z,rect,mode) = match t{
            Tile::Editor=>{
                let (t_loc,_) = tex_map.get_by_path("assets/cursor.png").ok_or("cursor.png not loaded")?;
				(6,shrink_by(r,10.),DrawMode::Tex(t_loc,UP,100.))
			},
            Tile::Man=>{
				let (t_loc,_) = match s.anims.get(gi){
                    Some(AnimState::Jmp)=>tex_map.get_by_path("assets/man_tr/man_03.png"),
                    Some(AnimState::RtJmp)=>tex_map.get_by_path("assets/man_tr/man_06.png"),
                    Some(AnimState::Rt)=>tex_map.get_by_path("assets/man_tr/man_05.png"),
                    _=>tex_map.get_by_path("assets/man_tr/man_01.png"),
                }.ok_or("Texture not loaded")?;

                (5,r, DrawMode::Tex(t_loc,s.gravity,50.))
            },
            Tile::Block=>{
				let (t_loc,_) = tex_map.get_by_path("assets/block.png").ok_or("man.png not loaded")?;
                (5,r, DrawMode::Tex(t_loc,UP,100.))
            },
        };
        s.draw.put(gi,DrawCp{z,mode,rect});
    }
    Ok(())
}

pub fn draw_sys(s:&State,tex_map:&TexLoader,c:Context,g:&mut G2d){
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

    for gi in ls_draw {
        if let Some(dc)=s.draw.get(gi){
            match dc.mode{
                DrawMode::Rect(rc)=> rc.draw(dc.rect, &draw_state::DrawState::new_alpha(),
                              c.transform,g),
				DrawMode::Tex(t_loc,t_ang,npx)=>{
					if let Some(tx) = tex_map.get(t_loc){
						image(tx,set_pos_angle(c.transform,dc.rect,t_ang,npx),g);
					}
				}
            }
        }
    }
}

pub fn grid_draw_sys(s:&State,tex_map:&TexLoader,c:Context,g:&mut G2d){
    let border = Border{color:[0.,0.,0.,1.],radius:3.0};
    Rectangle::new([1.,1.,1.,1.])
            .border(border)
            .draw([0.,0.,s.walls.w as f64* 50., s.walls.h as f64*50.],
                    &draw_state::DrawState::new_alpha(),
                    c.transform,g);
    for (i,w) in s.walls.v.iter().enumerate() {
        let x = (i as i32 % s.walls.w) as f64;
        let y = (i as i32 / s.walls.w) as f64;
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
                    if let Some((_,tx)) = 
                        tex_map.get_by_path("assets/spike.png") {
						image(tx,set_pos_angle(c.transform,[x1,y1,50.,50.],dr+2,100.),g);
                    }
                }
                Edge::Door=>{
                    if let Some((_,tx)) = 
                        tex_map.get_by_path("assets/door.png") {
						image(tx,set_pos_angle(c.transform,[x1,y1,50.,50.],dr+2,100.),g);
                    }
                }
                _=>{},
            }
        }
    }

    for (i,core) in s.walls.vc.iter().enumerate() {
        let x = (i as i32 % s.walls.w) as f64;
        let y = (i as i32 / s.walls.w) as f64;
        let (x1,y1) = (x*50.,y*50.);
        if let TileCore::GravChanger(dr) = core{
            if let Some((_,tx)) = 
                tex_map.get_by_path("assets/arrow.png") {
                image(tx,set_pos_angle(c.transform,[x1,y1,50.,50.],*dr,100.),g);

            }
        }
    }
}
