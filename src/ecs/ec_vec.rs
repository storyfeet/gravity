use crate::ecs::gen::GenItem;

#[derive(Clone,Debug)]
pub struct ECItem<T>{
    pub gen:u64,
    pub t:T,
}

impl<T> ECItem<T>{
    fn new(gen:u64,t:T)->Self{
        ECItem{gen,t}
    }
}

#[derive(Clone,Debug)]
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

    //TODO, fix to get a better iterator. 
    pub fn iter_mut<'a>(&'a mut self)->std::iter::Enumerate<std::slice::IterMut<'a, std::option::Option<ECItem<T>>>>{
        self.items.iter_mut().enumerate()
    }

    pub fn for_each<F:FnMut(GenItem,&mut T)>(&mut self,mut f:F){
        for (loc,v) in self.items.iter_mut().enumerate() {
            if let Some(ECItem{gen,t}) = v {
                f(GenItem{gen:*gen,loc},t);
            }
        }
    }

    pub fn compress(&mut self,cvec:Vec<(GenItem,GenItem)>){
        let mut res = ECVec::new();
        for (g_from,g_to) in cvec{
            if g_from.loc >= self.items.len(){ continue}
            if let Some(ECItem{gen,t}) = self.items[g_from.loc].take() {
                if gen == g_from.gen{ 
                    res.put(g_to,t);
                }
            }
        }
        *self = res;
    }
}

impl<'a,T> IntoIterator for &'a ECVec<T>{
    type Item = (GenItem,&'a T);
    type IntoIter = ECIter<'a,T>;
    fn into_iter(self)->Self::IntoIter{
        self.iter()
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


#[cfg(test)]
mod tests{
  

}


