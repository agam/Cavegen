// A "translation" from https://bitbucket.org/agambrahma/caves/

use array2d::Array2D;
use rand::Rng;

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

const WALL_PROB_PCT: u8 = 40;

fn main() {
    println!("Hello, world!");

    let mut grid: Array2D<Cell> = Array2D::filled_with(Cell::Space, NUM_ROWS, NUM_COLS);
    let mut rng = rand::thread_rng();

    // Display.
    let show_grid = || {
        for row_iter in grid.rows_iter() {
            for cell in row_iter {
                let cell_str = match cell {
                    Cell::Space => ".",
                    Cell::Wall => "#",
                };
                print!("{}", cell_str);
            }
            println!();
        }
    };

    // TODO: figure out why the "closure form" of this was harder to work with.
    fn seed_caves(grid: &mut Array2D<Cell>, rng: &mut rand::rngs::ThreadRng) {
        fn is_edge(row: usize, col: usize) -> bool {
            (row == 0) ||
                (col == 0) ||
                (row == NUM_ROWS - 1) ||
                (col == NUM_COLS - 1)
        }

        fn is_middle_row(row: usize) -> bool {
            row == (NUM_ROWS / 2)
        }

        fn should_place_wall(generator: &mut rand::rngs::ThreadRng) -> bool {
            generator.gen::<u8>() < WALL_PROB_PCT
        }

        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let newVal =
                    if is_edge(row, col) {
                        Cell::Wall
                    } else if is_middle_row(row) {
                        Cell::Space
                    } else if should_place_wall(rng) {
                        Cell::Wall
                    } else {
                        Cell::Space
                    };

                let result = grid.set(row, col, newVal);
                assert!(result.is_ok());
            }
        }
    };

    seed_caves(&mut grid, &mut rng);
    show_grid();
}
