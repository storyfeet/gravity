use serde_derive::{Serialize,Deserialize};

use crate::error::GravError;

use super::rects::Position;

pub type TileEdge = [Edge;4];

#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum Edge{
    Clear,
    Wall,
    Spike,
    Door,
}

#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum TileCore{
    Clear,
    GravChanger(usize),
}

pub fn can_pass(e_op:Option<Edge>)->bool{
    match e_op{
        Some(Edge::Door)|Some(Edge::Clear)=>true,
        _=>false,
    }
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct EdgeGrid{
    pub w:i32,
    pub h:i32,
    pub v:Vec<TileEdge>,
    pub vc:Vec<TileCore>,
}

impl EdgeGrid{
    pub fn new(w:i32,h:i32)->Self{ 
        let cap = (w*h)as usize;
        let mut v = Vec::with_capacity(cap);
        let mut vc = Vec::with_capacity(cap);
        for _ in 0..cap{
            v.push([Edge::Clear,Edge::Clear,Edge::Clear,Edge::Clear]);
            vc.push(TileCore::Clear);
        }
        EdgeGrid{w,h,v,vc}
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

    pub fn edge_at(&self,p:Position,dir:usize)->Option<Edge>{
        Some(self.v[self.pos_to_n(p)?][dir%4])
    }

    pub fn edge_at_mut(&mut self,p:Position,dir:usize)->Option<&mut Edge>{
        let n = self.pos_to_n(p)?;
        Some(&mut self.v[n][dir%4])
    }

    pub fn toggle_edge(&mut self, p:Position,dir:usize)->Result<(),GravError>{
        let edge = self.edge_at_mut(p,dir).ok_or("None at Grid Location")?;
        *edge = match *edge{
            Edge::Clear=>Edge::Wall,
            Edge::Wall=>Edge::Spike,
            Edge::Spike=>Edge::Door,
            Edge::Door=>Edge::Clear,
        };
        Ok(())
    }


    pub fn core_at(&self,p:Position)->Option<TileCore>{
        Some(self.vc[self.pos_to_n(p)?])
    }

    pub fn core_at_mut(&mut self,p:Position)->Option<&mut TileCore>{
        let n = self.pos_to_n(p)?;
        Some(&mut self.vc[n])
    }

    pub fn toggle_core(&mut self, p:Position)->Result<(),GravError>{
        let core = self.core_at_mut(p).ok_or("None at Grid Location")?;
        *core = match *core{
            TileCore::Clear=>TileCore::GravChanger(0),
            TileCore::GravChanger(3)=>TileCore::Clear,
            TileCore::GravChanger(n)=>TileCore::GravChanger(n+1),
        };
        Ok(())
    }

}


