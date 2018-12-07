use crate::ecs::gen::{GenManager,GenItem};
use crate::ecs::ec_vec::ECVec;


pub struct MenuState{
    focus:Option<GenItem>,
    gm:GenManager,
    buttons:ECVec<&'static str>,
}

impl MenuState{    
    pub fn new()->Self{
        MenuState{
            focus:None,
            gm:GenManager::new(),
            buttons:ECVec::new(),
        }
    }
}


