use crate::state::State;
use crate::ecs::ECItem;

pub fn move_sys(s:&mut State,d:f64){
    s.d_time += d; 
    if s.d_time > 2. {
        s.grid_pos.for_each(|gi,t|{
            t.x +=1;
        });
        s.d_time = 0.;
    }
}
