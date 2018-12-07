use crate::ecs::gen::{GenManager,GenItem};
use crate::ecs::ec_vec::ECVec;


pub struct MenuState{
    pub gm:GenManager,
    pub buttons:ECVec<&'static str>,
    pub focus:Option<GenItem>,
}

impl MenuState{    
    pub fn new()->Self{
        MenuState{
            gm:GenManager::new(),
            buttons:ECVec::new(),
            focus:None,
        }
    }

    pub fn add_button(&mut self,s:&'static str)->GenItem{
        let gi = self.gm.add();
        self.buttons.put(gi,s);
        gi
    }
}


