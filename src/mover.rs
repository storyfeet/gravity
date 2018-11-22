use crate::state::State;
use crate::ecs::ECItem;

pub fn move_sys(s:&mut State,d:f64){
    s.d_time += d; 
    if s.d_time > 2. {
        println!("{}",s.d_time);
        for (i,im) in s.grid_pos.iter_mut(){
            if let Some(ECItem{gen,t}) = im{
                t.x +=1
            }
        }
        s.d_time = 0.;
    }
}
