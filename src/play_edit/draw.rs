use piston_window::rectangle::{Rectangle,Border};
use piston_window::{image,Context,G2d,draw_state,line,Transformed};
use std::cmp::Ordering;

use crate::ecs::gen::GenItem;
use crate::error::GravError;
use crate::texture_loader::TexLoader;

use super::state::{State,Tile,DrawMode,DrawCp,AnimState};
use super::grid::{Edge,TileCore};
use super::rects::{Position,UP,DOWN,LEFT,RIGHT,shrink_by,QTrans};
//use graphics::transformed::Transformed;

const COL_BAD:[f32;4]= [0.9,0.1,0.1,1.];


struct DrawData{
    gi:GenItem,
    z:u8,
    t:Tile,
}

pub fn get_z(t:Tile)->u8{
    match t {
        Tile::Editor=>6,
        Tile::Man=>5,
        _=>0,
    }
}

pub fn tile_draw_sys(s:&State,tex_map:&TexLoader,c:Context,g:&mut G2d){
    let mut ls_draw:Vec<DrawData>= s.tiles.iter()
                                        .map(|(gi,&t)|DrawData{gi,z:get_z(t),t})
                                        .collect();

    ls_draw.sort_unstable_by(|a,b| a.z.cmp(&b.z));

    for ddat in ls_draw{
        let pos = match s.grid_pos.get(ddat.gi) {
            Some(p)=>p,None=>continue,
        };
        let dir = match ddat.t{
            Tile::Man=>s.gravity,
            _=>UP,
        };
        let (npx,flip,txf) = match ddat.t{
            Tile::Man=> match s.anims.get(ddat.gi){
                Some(AnimState::Lf)=>(50.,true,"assets/man_tr/man_03.png"),
                Some(AnimState::Rt)=>(50.,false,"assets/man_tr/man_03.png"),
                Some(AnimState::Jmp)=>(50.,false,"assets/man_tr/man_02.png"),
                Some(AnimState::LfJmp)=>(50.,true,"assets/man_tr/man_05.png"),
                Some(AnimState::RtJmp)=>(50.,false,"assets/man_tr/man_05.png"),
                _=>(50.,false,"assets/man_tr/man_00.png"),
            },
            Tile::Block=>(100.,false,"assets/block.png"),
            Tile::Editor=>(100.,false,"assets/cursor.png"),
        };
        let tx = match tex_map.get_by_path(txf){
            Some((_,t))=>t,None=>{
                println!("No texture for :{}",txf);
                continue;
            }
        };

        let c = c.transform
                    .trans(pos.x as f64 * 50.,pos.y as f64 *50.)
                    .dir_about(25.,25.,dir)
                    .scale(50./npx,50./npx);
        let c = if flip {
            c.trans(50.,0.).flip_h()
        }else {c};
        image(tx,c,g);
        
        
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

        for dir in 0..4{
            let npx= match w[dir]{
                Edge::Clear=>continue, 
                Edge::Wall=>50.,
                _=>100.,
            };

            let c = c.transform
                        .trans(x1,y1)
                        .dir_about(25.,25.,dir+2)
                        .scale(50./npx,50./npx);
            match w[dir]{
                Edge::Wall=>{
                    line([0.,0.,1.,1.],2.,[0.,50.,50.,50.],c,g);
                }
                Edge::Spike=>{
                    if let Some((_,tx)) = 
                        tex_map.get_by_path("assets/spike.png") {
						image(tx,c,g);
                    }
                }
                Edge::Door=>{
                    if let Some((_,tx)) = 
                        tex_map.get_by_path("assets/door.png") {
						image(tx,c,g);
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
        if let TileCore::GravChanger(dir) = core{
            if let Some((_,tx)) = 
                tex_map.get_by_path("assets/arrow.png") {
                let c = c.transform
                            .trans(x1,y1)
                            .dir_about(25.,25.,*dir)
                            .scale(0.5,0.5)
                            ;
                image(tx,c,g);

            }
        }
    }
}
