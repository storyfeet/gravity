use piston_window::Transformed;
use std::ops::Add;

pub const UP:usize = 0;
pub const RIGHT:usize = 1;
pub const DOWN:usize = 2;
pub const LEFT:usize = 3;

pub fn dir_as_deg(u:usize)->f64{
    (u as f64)*90.
}


#[derive(Copy,Clone,PartialEq,Debug)]
pub struct Position{
    pub x:i32,
    pub y:i32,
}

impl Position{
    pub fn new(x:i32,y:i32)->Self{
        Position{x,y}
    }

    pub fn from_dir(d:usize)->Self{
        match d%4{
            UP=>Position::new(0,-1),
            RIGHT=>Position::new(1,0),
            DOWN=>Position::new(0,1),
            _=>Position::new(-1,0),
        }
    }

}



impl Add for Position{
    type Output = Position;
    fn add(self,p:Self)->Self{
        Position{x:self.x + p.x,y:self.y+p.y}
    }
}

//DEPRECATING in favor of Position::from_dir
pub fn dir_as_pos(u:usize)->Position{
    Position::from_dir(u)
}



type Rect = [f64;4];

pub fn rot_about(x:f64,y:f64,cx:f64,cy:f64,ang:usize)->(f64,f64){
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

///Assumes [x,y,w,h]
pub fn shrink_by(r:Rect,n:f64)->Rect{
    [r[0]+n,r[1]+n,r[2] - (n*2.),r[3] - (n*2.)]
}


pub fn set_pos_angle(ct:[[f64; 3]; 2],r:Rect,ang:usize,npx:f64)->[[f64; 3]; 2]{
    ct
        .trans(r[0],r[1])
        .scale(r[2]/npx,r[3]/npx)
        .trans(r[2],r[3])
        .rot_deg(dir_as_deg(ang))
        .trans(-r[2],-r[3])
}



