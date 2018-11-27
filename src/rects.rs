use piston_window::Transformed;

pub const UP:usize = 0;
pub const RIGHT:usize = 1;
pub const DOWN:usize = 2;
pub const LEFT:usize = 3;

pub fn dir_as_deg(u:usize)->f64{
    (u as f64)*90.
}


type rect = [f64;4];

pub fn rot_about(x:f64,y:f64,cx:f64,cy:f64,ang:usize)->(f64,f64){
    //Assume UP to start;
    let (x,y) = (x-cx,y-cy);
    let (x,y) = match ang {
        LEFT=>(-y,x),
        DOWN=>(-x,-y),
        RIGHT=>(y,-x),
        _=>(x,y),
    };
    (x+cx,y+cy)
}

///Assumes [x,y,w,h]
pub fn shrink_by(r:rect,n:f64)->rect{
    [r[0]+n,r[1]+n,r[2] - (n*2.),r[3] - (n*2.)]
}




pub fn set_pos_angle(ct:[[f64; 3]; 2],r:rect,ang:usize)->[[f64; 3]; 2]{
    ct
        .trans(r[0],r[1])
        .scale(r[2]/100.,r[3]/100.)
        .trans(r[2],r[3])
        .rot_deg(dir_as_deg(ang))
        .trans(-r[2],-r[3])
}



