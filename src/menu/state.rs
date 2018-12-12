use crate::ecs::gen::{GenManager,GenItem};
use crate::ecs::ec_vec::ECVec;


pub struct MenuState{
    pub gm:GenManager,
    pub buttons:ECVec<&'static str>,
    pub texts:ECVec<String>,
    pub focus:Option<GenItem>,
    pub folder:String,
}

impl MenuState{    
    pub fn new(folder:String)->Self{
        MenuState{
            gm:GenManager::new(),
            buttons:ECVec::new(),
            texts:ECVec::new(),
            focus:None,
            folder,
        }
    }

    pub fn add_button(&mut self,s:&'static str)->GenItem{
        let gi = self.gm.add();
        self.buttons.put(gi,s);
        gi
    }
}


