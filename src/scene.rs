
pub enum SceneAction{
    Cont,
    DropOff,
    Quit,
    Replace(&'static str),
    Child(&'static str),
}


