use std::{
    cmp,
    str::FromStr,
    convert::Infallible
};

#[derive(Debug)]
struct Line {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

impl FromStr for Line {
    type Err = Infallible;

    // input: "x1,y1 -> x2,y2"
    // output: Line { x1, y1, x2, y2 }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coord: Vec<u16> = s
            .replace(" -> ", ",")
            .split(",")
            .filter_map(|i| i.parse().ok())
            .collect();

        Ok(Self {
            x1: coord[0],
            y1: coord[1],
            x2: coord[2],
            y2: coord[3],
        })
    }
}

pub fn main() {
    let lines: Vec<Line> = aoc::file_to_vec("data/2021/05.txt", "\n").unwrap();
    let mut diagram = [[[0 as u8; 2]; 990]; 990];
    let [mut result1, mut result2] = [0 as u16; 2];

    let [
        mut index_x,
        mut index_y,
        mut step_x,
        mut step_y
    ]: [isize; 4];

    for line in lines {
        if line.x1 == line.x2 {
            let min = cmp::min(line.y1, line.y2);
            let max = cmp::max(line.y1, line.y2);

            for i in min..=max {
                diagram[line.x1 as usize][i as usize][0] += 1;
            }
        } else if line.y1 == line.y2 {
            let min = cmp::min(line.x1, line.x2);
            let max = cmp::max(line.x1, line.x2);

            for i in min..=max {
                diagram[i as usize][line.y1 as usize][0] += 1;
            }
        } else {
            if line.x1 < line.x2 {
                step_x = 1;
            } else {
                step_x = -1;
            }

            if line.y1 < line.y2 {
                step_y = 1;
            } else {
                step_y = -1;
            }

            index_x = line.x1 as isize;
            index_y = line.y1 as isize;

            while index_x != line.x2 as isize + step_x && index_y != line.y2 as isize + step_y {
                diagram[index_x as usize][index_y as usize][1] += 1;
                index_x += step_x;
                index_y += step_y;
            }
        }
    }

    for i in 0..diagram.len() {
        for j in 0..diagram[i].len() {
            if diagram[i][j][0] > 1 {
                result1 += 1;
            }

            if diagram[i][j][0] + diagram[i][j][1] > 1 {
                result2 += 1;
            }
        }
    }

    println!("Result 1: {}", result1); // Result 1: 5092
    println!("Result 2: {}", result2); // Result 2: 20484
}
