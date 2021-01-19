// A "translation" from https://bitbucket.org/agambrahma/caves/

use array2d::Array2D;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Space,
    Wall
}

struct Params {
    r1_cutoff: u8,
    r2_cutoff: Option<i8>
}

const NUM_ROWS: usize = 5;
const NUM_COLS: usize = 7;

const WALL_PROB_PCT: u8 = 40;

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();

    // Display.
    fn show_grid(grid: & Array2D<Cell>) {
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
    fn seed_caves(rng: &mut rand::rngs::ThreadRng) -> Array2D<Cell> {
        let mut grid: Array2D<Cell> = Array2D::filled_with(Cell::Space, NUM_ROWS, NUM_COLS);
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
                let new_val =
                    if is_edge(row, col) {
                        Cell::Wall
                    } else if is_middle_row(row) {
                        Cell::Space
                    } else if should_place_wall(rng) {
                        Cell::Wall
                    } else {
                        Cell::Space
                    };

                let result = grid.set(row, col, new_val);
                assert!(result.is_ok());
            }
        }

        grid
    };

    // Helpers.
    fn abs(n: usize) -> usize {
        if n < 0 {
            0 - n
        } else {
            n
        }
    }

    fn get_neighbor_count(grid: & Array2D<Cell>, row: usize, col: usize, delta: usize) -> u8 {
        let mut count = 0;

        for i in (row - delta)..=(row+delta) {
            for j in (col - delta)..=(col+delta) {
                if i < 0 || i >= NUM_ROWS || j < 0 || j >= NUM_COLS {
                    continue
                }

                // Skip corners when delta > 1.
                let should_skip = delta > 1 && abs(i - row) == delta && (j - col) == delta;
                if !should_skip {
                    count = match grid.get(i, j).unwrap() {
                        Cell::Wall => count + 1,
                        Cell::Space => count,
                    }
                }
            }
        }

        count
    }

    fn apply_cell_rules(r1: i8, r2: i8, params: Params) -> Cell {
        if r1 >= params.r1_cutoff as i8 {
            return Cell::Wall
        }

        if r2 <= params.r2_cutoff.unwrap_or(-1) {
            return Cell::Wall
        }

        Cell::Space
    }



    // TODO: figure out why the mutable access-version of this was harder to work with.
    let grid = seed_caves(&mut rng);
    show_grid(&grid);
}
