use std::fs::File;

use std::io::{BufRead, BufReader};


#[derive(Debug)]
struct Grid {
    complete_idx: i64,
    data: Vec<Vec<(String, bool)>>,
    num_row: usize,
    num_col: usize,
    last_draw: String
}

impl Grid {
    fn new() -> Grid {
        Grid {complete_idx: -1, data: Vec::new(), num_row: 0, num_col: 0, last_draw: "".to_string()}
    }

    fn clone(&self, cell_adjuster: &dyn Fn((String, bool), i64)->(String, bool)) -> Self {
        let mut new_grid = Grid::new();
        new_grid.complete_idx = self.complete_idx;
        new_grid.num_row = self.data.len();
        new_grid.last_draw = self.last_draw.clone();
        for row in &self.data {
            let mut new_row = Vec::new();
            for cell in row {
                new_row.push(cell_adjuster(cell.to_owned(), new_grid.complete_idx));
            }
            new_grid.num_col = new_row.len();
            new_grid.data.push(new_row);
        }
        new_grid
    }

    fn add_row(&mut self, row: &Vec<&str>) {
        let row: Vec<(String, bool)> = row.iter().map(|v| (v.to_string(), false)).collect();
        self.num_row = self.data.len();
        self.num_col = row.len();
        self.data.push(row);
    }

    fn mark(&self, draw: &str) -> Grid {
        self.clone(&|cell: (String, bool), complete_idx: i64| 
            match cell {
                    (v, marked) if complete_idx != -1 => (v, marked),
                    (v, _marked) if v == draw => (v, true),
                    (v, marked) => (v, marked)
                })
    }

    fn test_and_set_complete(&self, complete_idx: i64, last_draw: &str) -> Grid {
        let mut new_grid = self.clone(&|cell, _complete_idx| cell);
        if new_grid.complete_idx != -1 {
            return new_grid;
        }
        let mut grid_win = false;
        // Rows
            for row in 0..new_grid.num_row {
                let mut row_win = true;
                for col in 0..new_grid.num_col {
                    if new_grid.data[row][col].1 != true {
                    row_win = false;
                }
            }
            if row_win {
                println!("row_win");
                grid_win = true;
            }
        }
        // Cols
        for col in 0..new_grid.num_col {
            let mut col_win = true;
            for row in 0..new_grid.num_row {
                if new_grid.data[row][col].1 != true {
                    col_win = false;
                }
            }
            if col_win {
                println!("col_win");
                grid_win = true;
            }
        }

        if grid_win {
            new_grid.complete_idx = complete_idx;
            new_grid.last_draw = last_draw.to_string();
        }
        new_grid
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn complete_index(&self) -> i64 {
        self.complete_idx
    }

    fn score(&self) -> i64 {
        let mut score = 0u64;
        for row in &self.data {
            for cell in row {
                if cell.1 != true {
                    println!("{:?}", cell);
                    score += cell.0.parse::<u64>().unwrap();
                }
            }
        }
        let draw_score = self.last_draw.parse::<u64>().unwrap();
        println!("score {} x {} = {}", score, draw_score, score * draw_score);
        0
    }
}

fn main() -> Result<(), std::io::Error> {

    let filename = "input";

    // Open the file in read-only mode (ignoring errors).

    let file = File::open(filename).unwrap();

    let mut reader = BufReader::new(file);

    let mut draws = String::new();

    reader.read_line(&mut draws)?;
    let draws: Vec<_> = draws.trim().split(',').collect();
    let mut grids = Vec::new();
    let mut current_grid = Grid::new();
    for line in reader.lines() {
        let line = line?.to_owned();
        let line: Vec<_> = line.split_whitespace().into_iter().collect();
        println!("line: {:?}", line);
        match line.len() {
            0 => {
                println!("New grid");
                if current_grid.len() > 0 {
                let closing_grid = current_grid;
                current_grid = Grid::new();
                grids.push(closing_grid);
                }
            },
            _ => {
                current_grid.add_row(&line);
            }
        }
    }
    grids.push(current_grid);
    println!("{:?} {:?}", draws, grids);

    for (idx, draw) in draws.iter().enumerate() {
        let mut new_grids = Vec::new();
        for grid in grids {
            let new_grid = grid.mark(draw);
            new_grids.push(new_grid);
        }
        grids = new_grids;
        let mut new_grids = Vec::new();
        for grid in grids {
            let new_grid = grid.test_and_set_complete(idx as i64, draw);
            new_grids.push(new_grid);
        }
        grids = new_grids;
        println!("{} {:?}", draw, grids);
    }

    // Get last closed grid
    let last_complete = grids.iter().max_by_key(|grid| grid.complete_index()).unwrap();
    last_complete.score();

    Ok(())
}
