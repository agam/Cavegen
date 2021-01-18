// A "translation" from https://bitbucket.org/agambrahma/caves/

use array2d::Array2D;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Space,
    Wall
}

struct Params {
    r1_cutoff: i32,
    r2_cutoff: Option<i32>
}

const NUM_ROWS: usize = 5;
const NUM_COLS: usize = 7;

const WALL_PROB_PCT: i32 = 40;

fn main() {
    println!("Hello, world!");

    let mut grid = Array2D::filled_with(Cell::Space, NUM_ROWS, NUM_COLS);

    let show_grid = || {
        for row_iter in grid.rows_iter() {
            for cell in row_iter {
                let cell_str = match cell {
                    Cell::Space => " ",
                    Cell::Wall => ".",
                };
                print!("{}", cell_str);
            }
            println!();
        }
    };

    show_grid();
    //for &mut cell in foo.iter_mut() {
        //println!("Debug: I got: {:?}", cell);
    //}
}
