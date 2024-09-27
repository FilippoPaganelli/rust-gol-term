use std::thread;
use std::time::Duration;

const COLS: usize = 20;
const ROWS: usize = 10;
const NUM_CELLS: usize = COLS * ROWS;
const ALIVE: char = 'R';
const DEAD: char = '_';

fn print_state(cells: &[char]) {
    for i in 0..ROWS {
        for j in 0..COLS {
            print!("{}", cells[i * COLS + j]);
        }
        print!("\n");
    }
}

fn init_glider(cells: &mut [char]) {
    cells[COLS * 1 + 4] = ALIVE;
    cells[COLS * 2 + 5] = ALIVE;
    cells[COLS * 3 + 3] = ALIVE;
    cells[COLS * 3 + 4] = ALIVE;
    cells[COLS * 3 + 5] = ALIVE;
}

fn next_state(cells: &mut [char]) {
    let mut next_cells: [char; NUM_CELLS] = [DEAD; NUM_CELLS];
    next_cells.copy_from_slice(&cells);

    for i in 1..ROWS - 1 {
        for j in 1..COLS - 1 {
            let k = j + i * COLS;
            let mut neighbours: u8 = 0;

            if cells[k + 1 + COLS] == ALIVE {
                neighbours += 1;
            }
            if cells[k + 1] == ALIVE {
                neighbours += 1;
            }
            if cells[k + 1 - COLS] == ALIVE {
                neighbours += 1;
            }
            if cells[k + 0 + COLS] == ALIVE {
                neighbours += 1;
            }
            if cells[k + 0 - COLS] == ALIVE {
                neighbours += 1;
            }
            if cells[k - 1 + COLS] == ALIVE {
                neighbours += 1;
            }
            if cells[k - 1] == ALIVE {
                neighbours += 1;
            }
            if cells[k - 1 - COLS] == ALIVE {
                neighbours += 1;
            }

            if cells[k] == ALIVE {
                if neighbours < 2 || neighbours > 3 {
                    next_cells[k] = DEAD;
                }
            }
            if cells[k] == DEAD {
                if neighbours == 3 {
                    next_cells[k] = ALIVE;
                }
            }
        }
    }

    cells.copy_from_slice(&next_cells);
}

fn main() {
    let mut cells: [char; NUM_CELLS] = [DEAD; NUM_CELLS];
    init_glider(&mut cells);

    loop {
        print_state(&cells);

        print!("\x1b[{}A", ROWS);
        print!("\x1b[{}D", COLS);

        next_state(&mut cells);

        thread::sleep(Duration::from_millis(500));
    }
}
