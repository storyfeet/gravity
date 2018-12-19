use std::fs::File;
use std::path::Path;

use mksvg::{Args, PathD, SvgArg, SvgIO, SvgWrite};

use crate::error::GravError;
use crate::play_edit::grid::{Edge, TileCore};
use crate::play_edit::saver::LevelSave;
use crate::play_edit::state::Tile;

pub fn svg_out(l_sv: &LevelSave, path: &Path, imgpath: &Path) -> Result<(), GravError> {
    let f = File::create(path).map_err(|_| "Could not create")?;

    let mut s = SvgIO::new(f);

    let walls = &l_sv.walls;

    s.start(walls.w * 50, walls.h * 50);

    s.rect(
        0,
        0,
        walls.w * 50,
        walls.h * 50,
        Args::new().stroke("black").fill("white").stroke_width(4),
    );

    //walls / edges
    for (i, v) in walls.v.iter().enumerate() {
        let x = i as i32 % walls.w;
        let y = i as i32 / walls.w;
        s.g_translate(x * 50, y * 50);
        for dir in 0..4 {
            s.g_rotate((dir + 2) * 90, 25, 25);
            match v[dir] {
                Edge::Wall => {
                    s.path(
                        PathD::abs().m(0, 50).l(50, 50),
                        Args::new().stroke("red").stroke_width(3),
                    );
                }
                Edge::Spike => s.img(
                    &imgpath
                        .join("spike.png")
                        .to_str()
                        .expect("Path to string fail"),
                    0,
                    0,
                    50,
                    50,
                ),
                Edge::Door => s.img(
                    &imgpath
                        .join("door.png")
                        .to_str()
                        .expect("Path to string fail"),
                    2,
                    2,
                    46,
                    46,
                ),
                Edge::Clear => {}
            }
            s.g_end();
        }

        s.g_end();
    }

    //tile middles
    for (i, v) in walls.vc.iter().enumerate() {
        let x = i as i32 % walls.w;
        let y = i as i32 / walls.w;
        s.g_translate(x * 50, y * 50);
        match v {
            TileCore::GravChanger(n) => {
                s.g_rotate(n * 90, 25, 25);
                s.img(
                    &imgpath
                        .join("arrow.png")
                        .to_str()
                        .expect("Path to string fail"),
                    2,
                    2,
                    46,
                    46,
                );
                s.g_end();
            }
            _ => {}
        }
        s.g_end();
    }

    for (gi, t) in &l_sv.tiles {
        let p = l_sv
            .positions
            .get(gi)
            .expect("Unexpected Tile with no position");

        s.g_translate(p.x * 50, p.y * 50);
        let fname = match t {
            Tile::Man => "man_tr/man_00.png",
            Tile::Block => "block.png",
            _ => "cursor.png",
        };
        s.img(
            &imgpath.join(fname).to_str().expect("Path to string fail"),
            5,
            5,
            40,
            40,
        );
        s.g_end();
    }
    s.end();
    Ok(())
}
