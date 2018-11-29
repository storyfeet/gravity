use crate::rects::Position;
use crate::error::GravError;


pub type TileEdge = [Edge;4];

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Edge{
    Clear,
    Wall,
    Spike,
    Door,
}

pub fn can_pass(e_op:Option<Edge>)->bool{
    match e_op{
        Some(Edge::Door)|Some(Edge::Clear)=>true,
        _=>false,
    }
}

pub struct EdgeGrid{
    pub w:i32,
    pub h:i32,
    pub v:Vec<TileEdge>,
}

impl EdgeGrid{
    pub fn new(w:i32,h:i32)->Self{ 
        let cap = (w*h)as usize;
        let mut v = Vec::with_capacity(cap);
        for _ in 0..cap{
            v.push([Edge::Clear,Edge::Clear,Edge::Clear,Edge::Clear]);
        }
        EdgeGrid{w,h,v}
    }

    pub fn pos_to_n(&self,p:Position)->Option<usize>{
        if p.x < 0 { return None}
        if p.y < 0 { return None}
        if p.x >= self.w { return None }
        if p.y >= self.h { return None }
        let res = (p.x + p.y * self.w)as usize;
        if res > self.v.len() { return None}
        return Some(res);
    }

    pub fn at(&self,p:Position,dir:usize)->Option<Edge>{
        Some(self.v[self.pos_to_n(p)?][dir%4])
    }

    pub fn at_mut(&mut self,p:Position,dir:usize)->Option<&mut Edge>{
        let n = self.pos_to_n(p)?;
        Some(&mut self.v[n][dir%4])
    }


    pub fn toggle_wall(&mut self, p:Position,dir:usize)->Result<(),GravError>{
        let edge = self.at_mut(p,dir).ok_or("None at Grid Location")?;
        *edge = match *edge{
            Edge::Clear=>Edge::Wall,
            Edge::Wall=>Edge::Spike,
            Edge::Spike=>Edge::Door,
            Edge::Door=>Edge::Clear,
        };
        Ok(())
    }

}


