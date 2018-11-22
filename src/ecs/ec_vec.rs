use crate::ecs::gen::GenItem;

struct ECItem<T>{
    gen:u64,
    t:T,
}

impl<T> ECItem<T>{
    fn new(gen:u64,t:T)->Self{
        ECItem{gen,t}
    }
}

pub struct ECVec<T>{
    items:Vec<Option<ECItem<T>>>
}

impl<T> ECVec<T>{
    pub fn new()->Self{
        ECVec{
            items:Vec::new(),
        }
    }
    pub fn put(&mut self,k:GenItem,v:T){
        while self.items.len() <= k.loc{
            self.items.push(None);
        }
        self.items[k.loc]= Some(ECItem::new(k.gen,v));
    }
    pub fn drop(&mut self,k:GenItem){
        if self.items.len() <= k.loc {return}
        self.items[k.loc] = None;
    }

    pub fn get_mut(&mut self,k:GenItem)->Option<&mut T>{
        if self.items.len() < k.loc{ return None}
        match self.items[k.loc] {
            Some(ref mut ecit)=>{
                (if ecit.gen == k.gen {
                    Some(&mut ecit.t)
                }else{
                    None
                })
            }
            _=>None,
        }
    }

    pub fn get(&self,k:GenItem)->Option<&T>{
        if self.items.len() < k.loc{ return None}
        match self.items[k.loc] {
            Some(ref ecit)=>{
                (if ecit.gen == k.gen {
                    Some(&ecit.t)
                }else{
                    None
                })
            }
            _=>None,
        }
    }

    pub fn iter<'a>(&'a self)->ECIter<'a,T>{
        ECIter{
            n:0,
            v:self,
        }
    }
}

pub struct ECIter<'a,T>{
    n:usize,
    v:&'a ECVec<T>,
}

impl<'a,T> Iterator for ECIter<'a,T>
{
    type Item=(GenItem,&'a T);
    fn next(&mut self)->Option<(GenItem,&'a T)>{
        while self.n < self.v.items.len(){
            self.n+=1;
            if let Some(ref ecit) = self.v.items[self.n-1]{
                return Some( (GenItem{loc:self.n-1,gen:ecit.gen}, &ecit.t));
            }
        }
        return None;
    }
}

