use piston_window::{PistonWindow,WindowSettings,Event,Loop,clear,draw_state};

use piston_window::rectangle::{Rectangle,Border};


mod ecs;
mod state;

fn main() {

    let mut g_state = state::State::new();

    g_state.add_tile(state::Tile::Man,state::Position{x:2,y:4});

    

    let mut window:PistonWindow = 
        WindowSettings::new("Gravity",[640,480])
                    .exit_on_esc(true)
                    .build()
                    .unwrap();
    while let Some(e) = window.next(){        
        window.draw_2d(&e,|c,g|{
            clear([1.,0.,0.,1.],g);
            let border = Border{color:[0.,0.,0.,1.],radius:5.};
            for (gi,pi) in g_state.positions.iter(){
                Rectangle::new([0.0,1.,0.,0.1])
                    .border(border)
                    .draw([(pi.x*50) as f64,(pi.y*50) as f64,50.,50.],
                          &draw_state::DrawState::new_alpha(),
                          c.transform,g);
            }
            
        });
        match e {
            
            Event::Loop(Loop::Update(d))=>{
                //println!("Udate {:?}",d);
            },
            _=>{},//println!("OTHER {:?}",e),
        }

    }
}
