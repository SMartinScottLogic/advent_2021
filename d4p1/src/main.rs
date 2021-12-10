use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let filename = "input";

    // Open the file in read-only mode (ignoring errors).

    let file = File::open(filename).unwrap();

    let mut reader = BufReader::new(file);

    let mut draws = String::new();

    reader.read_line(&mut draws)?;
    let draws: Vec<_> = draws.trim().split(',').collect();
    let mut grids = Vec::new();
    let mut current_grid = Vec::new();
    for line in reader.lines() {
        let line = line?.to_owned();
        let line: Vec<_> = line
            .split_whitespace()
            .into_iter()
            .map(|v| (v.to_owned(), false))
            .collect();
        println!("line: {:?}", line);
        match line.len() {
            0 => {
                println!("New grid");
                if current_grid.len() > 0 {
                    let closing_grid = current_grid;
                    current_grid = Vec::new();
                    grids.push(closing_grid);
                }
            }
            _ => {
                current_grid.push(line);
            }
        }
    }
    grids.push(current_grid);
    println!("{:?} {:?}", draws, grids);

    for draw in draws {
        let mut new_grids = Vec::new();
        for grid in grids {
            let mut new_grid = Vec::new();
            for row in grid {
                let mut new_row = Vec::new();
                for cell in row {
                    let new_cell = match cell {
                        (v, _marked) if v == draw => (v, true),
                        (v, marked) => (v, marked),
                    };
                    new_row.push(new_cell);
                }
                new_grid.push(new_row);
            }
            new_grids.push(new_grid);
        }
        grids = new_grids;
        for grid in &grids {
            let mut grid_win = false;
            let num_row = grid.len();
            let mut num_col = 0;
            for row in grid {
                let mut row_win = true;
                num_col = row.len();
                for cell in row {
                    if cell.1 != true {
                        row_win = false;
                    }
                }
                if row_win {
                    println!("row_win");
                    grid_win = true;
                }
            }
            for col in 0..num_col {
                let mut col_win = true;
                for row in 0..num_row {
                    if grid[row][col].1 != true {
                        col_win = false;
                    }
                }
                if col_win {
                    println!("col_win");
                    grid_win = true;
                }
            }
            if grid_win {
                let mut score = 0u64;
                for row in grid {
                    for cell in row {
                        if cell.1 != true {
                            println!("{:?}", cell);
                            score += cell.0.parse::<u64>().unwrap();
                        }
                    }
                }
                let draw_score = draw.parse::<u64>().unwrap();
                println!("score {} x {} = {}", score, draw_score, score * draw_score);
                panic!("done");
            }
        }
        println!("{} {:?}", draw, grids);
    }

    Ok(())
    /*
    let mut oxygen: Vec<String> = data.iter().map(|v| v.clone()).collect();
    let mut index = 0;
    while oxygen.len() > 1
    {
        let p = oxygen.iter().fold(HashMap::new(), |mut acc, v| {
            *acc.entry(v.as_bytes()[index] as char).or_insert(0) += 1;
            acc
        });
        let mut most_common_value = p.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k).unwrap();
        if p.get(&'0')!=None && p.get(&'1')!=None && p[&'0'] == p[&'1'] {
            most_common_value = &'1';
        }
        println!("{:?} {}", p, most_common_value);
        let next_oxygen: Vec<String> = oxygen.iter().filter(|v| v.as_bytes()[index] as char == *most_common_value).map(|v| v.to_owned()).collect();
        println!("next: {} {:?}", index, next_oxygen);
        oxygen = next_oxygen;
        index += 1;
    }
    let mut co2: Vec<String> = data.iter().map(|v| v.clone()).collect();
    let mut index = 0;
    while co2.len() > 1
    {
        let p = co2.iter().fold(HashMap::new(), |mut acc, v| {
            *acc.entry(v.as_bytes()[index] as char).or_insert(0) += 1;
            acc
        });
        let mut least_common_value = p.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k).unwrap();
        if p.get(&'0')!=None && p.get(&'1')!=None && p[&'0'] == p[&'1'] {
            least_common_value = &'0';
        }
        println!("{:?} {}", p, least_common_value);
        let next_co2: Vec<String> = co2.iter().filter(|v| v.as_bytes()[index] as char == *least_common_value).map(|v| v.to_owned()).collect();
        println!("next: {} {:?}", index, next_co2);
        co2 = next_co2;
        index += 1;
    }
    let oxygen = i32::from_str_radix(&oxygen.get(0).unwrap(), 2).unwrap();
    let co2 = i32::from_str_radix(&co2.get(0).unwrap(), 2).unwrap();

    println!("{} x {} = {}", oxygen, co2, oxygen * co2);
    */
    /*
    let mut last: Option<i32> = None;
    let mut count = 0;
    let mut counts = HashMap::new();

    for line in reader.lines() {
        count += 1;
        let line = line.unwrap();
        let mut reading = u32::from_str_radix(&line, 2).unwrap();

        println!("{} -> {}", line, reading);
        let mut index = 0;
        while reading > 0 {
            let field = reading & 1;
            println!("  {} {}", index, field);
            *counts.entry(index).or_insert(0) += field;
            reading >>= 1;
            index += 1;
        }

        /*
        let reading: i32 = line.parse().unwrap();

        match last {
            Some(v) if reading > v => count += 1,
            _ => {}
        }
        // Show the line and its number.

        println!("{}. {}", index + 1, reading);
        last = Some(reading);
        */

    }
    println!("Count = {} {:?}", count, counts);

    let mut epsilon = 0;
    let mut gamma = 0;
    for (k, v) in counts {
        if v > (count >> 1) {
          let value = 1 << k;
          epsilon += value;
        } else {
          let value = 1 << k;
          gamma += value;
        }

    }
    println!("total = {} x {} = {}", epsilon, gamma, epsilon * gamma);
    */
}
