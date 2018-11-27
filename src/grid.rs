use crate::state::Position;


pub type TileEdge = [Edge;4];

pub enum Edge{
    Clear,
    Wall,
    Spike,
    Door,
}

pub struct EdgeGrid{
    pub w:usize,
    pub h:usize,
    pub v:Vec<TileEdge>,
}

impl EdgeGrid{
    pub fn new(w:usize,h:usize)->Self{ 
        let cap = w*h;
        let mut v = Vec::with_capacity(cap);
        for _ in 0..cap{
            v.push([Edge::Clear,Edge::Clear,Edge::Clear,Edge::Clear]);
        }
        EdgeGrid{w,h,v}
    }

    pub fn toggle_wall(&mut self, p:Position,dir:usize){
        let (x,y) =( p.x as usize, p.y as usize);
        if x >= self.w { return }
        if y >= self.h { return }
        let n = x + y* self.w;
        if n >= self.v.len() {return}
        if dir >3 {return}
        self.v[n][dir] = match self.v[n][dir]{
            Edge::Clear=>Edge::Wall,
            Edge::Wall=>Edge::Spike,
            Edge::Spike=>Edge::Door,
            Edge::Door=>Edge::Clear,
        };
    }

}


