use std::path::Path;
use std::fs::File;

use mksvg::{SvgIO,SvgWrite};

use crate::play_edit::state::State;
use crate::error::GravError;


pub fn svg_out(st:&State,path:&Path,imgpath:&str)->Result<(),GravError>{
    let f = File::create(path).map_err(|_|"Could not create")?;

    let mut s = SvgIO::new(f);
    s.start(300.,300.);
    s.end();

    Ok(())
    
}
