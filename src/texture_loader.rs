//use std::sync::{Arc,Mutex};
use std::collections::BTreeMap;

use piston_window::{G2dTexture,Flip,TextureSettings};

pub struct TexLoader{
    v:Vec<G2dTexture>,
    map:BTreeMap<String,usize>,
}

//consider using GenManager and GenItem if drops become common
impl TexLoader{
    pub fn new()->Self{
        TexLoader{
            v:Vec::new(),
            map:BTreeMap::new(),
        }
    }

    pub fn load<F>(&mut self,fac:&mut F ,fname:&str)->Result<usize,String>
		where F: gfx_core::factory::Factory<gfx_device_gl::Resources>
    {     

        match self.map.get(fname) {
            Some(i)=>Ok(*i),
            None=>{
                let t = G2dTexture::from_path(fac,fname,Flip::None,&TextureSettings::new())?;
                let vloc = self.v.len();
                self.v.push(t); 
                self.map.insert(fname.to_string(),vloc);
                Ok(vloc)
            }
        }
    }

    pub fn get(&self,n:usize)->Option<&G2dTexture>{
        self.v.get(n) 
    }
}


