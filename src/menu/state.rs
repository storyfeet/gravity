use crate::ecs::ec_vec::ECVec;
use crate::ecs::gen::{GenItem, GenManager};

pub struct MenuState {
    pub gm: GenManager,
    pub buttons: ECVec<&'static str>,
    pub texts: ECVec<String>,
    pub focus: Option<GenItem>,
    pub folder: String,
    pub svg_link: String,
}

impl MenuState {
    pub fn new(folder: String, svg_link: String) -> Self {
        MenuState {
            gm: GenManager::new(),
            buttons: ECVec::new(),
            texts: ECVec::new(),
            focus: None,
            folder,
            svg_link,
        }
    }

    pub fn add_button(&mut self, s: &'static str) -> GenItem {
        let gi = self.gm.add();
        self.buttons.put(gi, s);
        gi
    }
}
