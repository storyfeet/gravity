use std::fs::File;
use std::path::Path;

use mksvg::{Args, PathD, SvgArg, SvgIO, SvgWrite};

use crate::error::GravError;
use crate::play_edit::grid::Edge;
use crate::play_edit::saver::LevelSave;

pub fn svg_out(l_sv: &LevelSave, path: &Path, imgpath: &str) -> Result<(), GravError> {
    let f = File::create(path).map_err(|_| "Could not create")?;

    let mut s = SvgIO::new(f);

    let walls = &l_sv.walls;

    s.start(walls.w * 50, walls.h * 50);

    s.rect(
        0,
        0,
        walls.w * 50,
        walls.h * 50,
        Args::new().stroke("black").fill("white"),
    );

    for (i, v) in walls.v.iter().enumerate() {
        let x = i as i32 % walls.w;
        let y = i as i32 / walls.w;
        let xp = x * 50;
        let yp = y * 50;
        for dir in 0..4 {
            match v[dir] {
                Edge::Wall => {
                    s.path(
                        PathD::abs().m(xp, yp).l(xp + 50, yp + 50),
                        Args::new().stroke("black"),
                    );
                }
                _ => {}
            }
        }
    }

    s.end();

    Ok(())
}
