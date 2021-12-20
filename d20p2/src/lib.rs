use anyhow::{Context, Result};
use log::{debug, trace};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).with_context(|| format!("Failed to read from {}", filename))?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    let mut image = Image::new();

    for (line_no, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();

        if line_no == 0 {
            if line.len() != 512 {
                panic!();
            }
            solution.set_algorithm(line);
            continue;
        }
        if line.is_empty() {
            continue;
        }
        image = image + line;
    }
    solution.set_image(image);

    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    outside: char,
    algorithm: String,
    image: Image,
    answer: i64,
}

impl Solution {
    fn new() -> Self {
        Self {
            outside: '.',
            ..Default::default()
        }
    }

    pub fn analyse(&mut self) {
        for _pass in 1..=50 {
            self.dump("start");
            self.apply_algorithm();
            self.dump("end");
        }
        self.answer = self.count_lit();
    }

    pub fn answer(&self) -> Result<i64> {
        Ok(self.answer)
    }
}

impl Solution {
    fn count_lit(&self) -> i64 {
        let mut count = 0;
        for y in 0..self.image.height() {
            for x in 0..self.image.width() {
                count += match self.image.get(x.try_into().unwrap(), y.try_into().unwrap()) {
                    '#' => 1,
                    '.' => 0,
                    _ => unreachable!(),
                }
            }
        }
        count
    }

    fn apply_algorithm(&mut self) {
        let growth_factor: i64 = 2;

        let mut new_image = Image::new();

        let max_x = self.image.width() as i64;
        let max_y = self.image.height() as i64;
        for y in 0..(max_y + growth_factor * 2) {
            let mut new_line = String::new();
            for x in 0..(max_x + growth_factor * 2) {
                let mut key = String::new();
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let original_x = x + dx - growth_factor;
                        let original_y = y + dy - growth_factor;
                        if original_x < 0
                            || original_y < 0
                            || original_x >= max_x
                            || original_y >= max_y
                        {
                            key.push(match self.outside {
                                '.' => '0',
                                '#' => '1',
                                _ => unreachable!(),
                            });
                        } else {
                            let source = match self.image.get(original_x, original_y) {
                                '.' => '0',
                                '#' => '1',
                                _ => unreachable!(),
                            };
                            key.push(source);
                        }
                    }
                }
                if key.len() != 9 {
                    panic!();
                }
                let key = usize::from_str_radix(&key, 2).unwrap();
                trace!("{} {}", key, self.algorithm.len());
                let new_pixel = self.algorithm.chars().nth(key).unwrap();
                trace!("({}, {}) {} -> {}", x, y, key, new_pixel);
                new_line.push(new_pixel);
            }
            debug!("{}", new_line);
            new_image = new_image + &new_line;
        }
        self.image = new_image;
        let key = match self.outside {
            '#' => "111111111",
            '.' => "000000000",
            _ => unreachable!(),
        };
        let key = usize::from_str_radix(key, 2).unwrap();
        self.outside = self.algorithm.chars().nth(key).unwrap();
    }

    fn dump(&self, stage: &str) {
        debug!("======");
        debug!(" {}", stage);
        debug!(" outside: {}", self.outside);
        self.image.dump();
        debug!("======");
    }

    fn set_algorithm(&mut self, algorithm: &str) {
        self.algorithm = algorithm.to_string();
    }

    fn set_image(&mut self, image: Image) {
        self.image = image;
    }
}
#[derive(Debug, Default)]
struct Image {
    data: Vec<String>,
}

impl Image {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn width(&self) -> usize {
        self.data.get(0).unwrap().len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn dump(&self) {
        for line in &self.data {
            debug!("{}", line);
        }
    }

    fn get(&self, x: i64, y: i64) -> char {
        trace!("get({}, {})", x, y);
        trace!("  max_y = {}", self.data.len());
        let line = &self.data[y as usize];
        trace!("  max_x = {}", line.len());
        line.chars().nth(x as usize).unwrap()
    }
}

impl Add<&str> for Image {
    type Output = Self;

    fn add(mut self, other: &str) -> Self {
        self.data.push(other.to_string());
        self
    }
}
