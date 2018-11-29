pub enum GravError{
    Mess(String),
}
use self::GravError::*;


impl From<&str>for GravError{
    fn from(s:&str)->Self{
        Mess(s.to_string())
    }
}