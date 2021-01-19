// A "translation" from https://bitbucket.org/agambrahma/caves/

use array2d::Array2D;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Space,
    Wall
}

#[derive(Debug, Clone, Copy)]
struct Params {
    r1_cutoff: u8,
    r2_cutoff: Option<i8>
}

const NUM_ROWS: usize = 10;
const NUM_COLS: usize = 20;

const WALL_PROB_PCT: u8 = 40;

fn main() {
    println!("Hello, world!");

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
            generator.gen_range(0..100) < WALL_PROB_PCT
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
    fn abs(n1: usize, n2: usize) -> usize {
        if n1 > n2 {
            n1 - n2
        } else {
            n2 - n1
        }
    }

    fn get_neighbor_count(grid: & Array2D<Cell>, row: usize, col: usize, delta: usize) -> i8 {
        let mut count = 0;

        let startx = if col > delta { col } else { 0 };
        let stopx = std::cmp::min(col + delta, NUM_COLS - 1);

        let starty = if row > delta { row } else { 0 };
        let stopy = std::cmp::min(row + delta, NUM_ROWS - 1);

        for i in starty..=stopy {
            for j in startx..=stopx {

                // Skip corners when delta > 1.
                let should_skip = delta > 1 && abs(i, row) == delta && abs(j, col) == delta;
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

    fn apply_cell_rules(r1: i8, r2: i8, params: &Params) -> Cell {
        if r1 >= params.r1_cutoff as i8 {
            return Cell::Wall
        }

        if r2 <= params.r2_cutoff.unwrap_or(-1) {
            return Cell::Wall
        }

        Cell::Space
    }

    fn iterate_caves(old_grid: &Array2D<Cell>, params: &Params) -> Array2D<Cell> {
        let mut new_grid: Array2D<Cell> = Array2D::filled_with(Cell::Space, NUM_ROWS, NUM_COLS);
        
        // Create outer boundary.
        for i in [0, NUM_ROWS-1].iter() {
            for j in 0..NUM_COLS {
                assert!(new_grid.set(*i, j, Cell::Wall).is_ok());
            }
        }
        for j in [0, NUM_COLS-1].iter() {
            for i in 0..NUM_ROWS {
                assert!(new_grid.set(i, *j, Cell::Wall).is_ok());
            }
        }

        // Transform each cell.
        for i in 1..(NUM_ROWS - 1) {
            for j in 1 ..(NUM_COLS - 1) {
                let num_neighbors_1 = get_neighbor_count(old_grid, i, j, 1);
                let num_neighbors_2 = get_neighbor_count(old_grid, i, j, 2);

                let new_cell = apply_cell_rules(num_neighbors_1, num_neighbors_2, params);
                //println!("DebugAgam: at {:?}, {:?}, old = {:?}, neghbors = {:?}, {:?}, new = {:?}", i, j, old_grid.get(i, j).unwrap(), num_neighbors_1, num_neighbors_2, new_cell);

                assert!(new_grid.set(i, j, new_cell).is_ok());
            }
        }

        new_grid
    }

    fn run() {
        let mut rng = rand::thread_rng();

        // TODO: figure out why the mutable access-version of this was harder to work with.
        let mut grid = seed_caves(&mut rng);
        show_grid(&grid);


        // Two rounds of coalescing, larger then smaller islands.
        let round_1 = Params {
            r1_cutoff: 5,
            r2_cutoff: Some(6),
        };
        let round_2 = Params {
            r1_cutoff: 5,
            r2_cutoff: Some(6),
        };

        // One round of pruning out the isolated walls.
        let round_3 = Params {
            r1_cutoff: 5,
            r2_cutoff: None,
        };

        // TODO: populate params vector properly.
        let params_list = vec![
            round_1, round_1, round_1, round_1,
            round_2, round_2, round_2,
            round_3
        ];

        for (i, params) in params_list.iter().enumerate() {
            println!("\nIteration #{}\n--------------------\n\n", i);
            let new_grid = iterate_caves(&grid, &params);
            show_grid(&new_grid);
            grid = new_grid;
        }
    }

    run()
}
