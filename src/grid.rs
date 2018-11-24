use crate::state::Position;

pub const UP:usize = 0;
pub const LEFT:usize = 1;
pub const DOWN:usize = 2;
pub const RIGHT:usize = 3;

pub type TileWall = [Wall;4];

pub enum Wall{
    Clear,
    Line,
    Spike,
}

pub struct WallGrid{
    pub w:usize,
    pub h:usize,
    pub v:Vec<TileWall>,
}

impl WallGrid{
    pub fn new(w:usize,h:usize)->Self{ 
        let cap = w*h;
        let mut v = Vec::with_capacity(cap);
        for _ in 0..cap{
            v.push([Wall::Clear,Wall::Clear,Wall::Clear,Wall::Clear]);
        }
        WallGrid{w,h,v}
    }

    pub fn toggle_wall(&mut self, p:Position,dir:usize){
        let (x,y) =( p.x as usize, p.y as usize);
        if x >= self.w { return }
        if y >= self.h { return }
        let n = x + y* self.w;
        if n >= self.v.len() {return}
        if dir >3 {return}
        self.v[n][dir] = match self.v[n][dir]{
            Wall::Clear=>Wall::Line,
            Wall::Line=>Wall::Spike,
            Wall::Spike=>Wall::Clear,
        };
    }

}


