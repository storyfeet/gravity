
#[derive(Copy,Clone,PartialEq,Debug)]
pub struct GenItem{
    pub loc:usize,
    pub gen:u64,
}


pub struct GenManager{
    c_gen:u64,
    items:Vec<Option<u64>>,
}

impl GenManager{
    pub fn new()->Self{
        GenManager{
            c_gen:0,
            items:Vec::new(),
        }
    }

    pub fn add(&mut self)->GenItem{
        for (i,v) in (&mut self.items).into_iter().enumerate() {
            if *v == None { 
                *v = Some(self.c_gen);
                let res = GenItem{gen:self.c_gen,loc:i};
                self.c_gen +=1;
                return res;
            }
        }
        self.items.push(Some(self.c_gen));
        let res = GenItem{gen:self.c_gen,loc:self.items.len()-1};
        self.c_gen +=1;
        res
    }

    pub fn drop_item(&mut self,gi:GenItem){
        if self.items.len() <= gi.loc { return }
        if let Some(n)= self.items[gi.loc]{
            if n == gi.gen {
                self.items[gi.loc] = None
            }
        }
    }
}
