

#[derive(Debug,Clone,PartialEq)]
pub enum GravError{
    NonError,
    Mess(String),
}
use self::GravError::*;


impl From<&str>for GravError{
    fn from(s:&str)->Self{
        Mess(s.to_string())
    }
}

