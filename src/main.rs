// A "translation" from https://bitbucket.org/agambrahma/caves/

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

    let mut foo: [Cell; NUM_COLS] = [Cell::Space; NUM_COLS];

    for &mut cell in foo.iter_mut() {
        println!("Debug: I got: {:?}", cell);
    }
}
