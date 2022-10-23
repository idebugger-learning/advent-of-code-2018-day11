use std::collections::HashMap;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;
// const SERIAL_NUMBER: i32 = 42;
const SERIAL_NUMBER: i32 = 7403;

fn main() {
    let grid = init_grid();
    println!("{:?}", grid);
    println!("Calculatin part one...");
    let sums = square_sum(&grid);
    println!("Normalizing part one...");
    let (x, y, pwr) = sums
        .into_iter()
        .map(|row| {
            row.into_iter()
                .enumerate()
                .max_by_key(|(_, pwr)| *pwr)
                .unwrap()
        })
        .enumerate()
        .max_by_key(|(_, (_, pwr))| *pwr)
        .map(|(y, (x, pwr))| (x + 1, y + 1, pwr))
        .unwrap();
    println!("Max power at {}, {}: {}", x, y, pwr);

    println!("Calculatin part two...");
    let square_sums = sums_of_all_squares(&grid);
    println!("Normalizing part two...");
    let (x, y, size, pwr) = square_sums
        .into_iter()
        .map(|row| {
            row.into_iter()
                .enumerate()
                .max_by_key(|(_, (_, pwr))| *pwr)
                .unwrap()
        })
        .enumerate()
        .max_by_key(|(_, (_, (_, pwr)))| *pwr)
        .map(|(y, (x, (size, pwr)))| (x + 1, y + 1, size, pwr))
        .unwrap();
    println!("Square max sum at {}, {}, size {}: {}", x, y, size, pwr);
}

fn init_grid() -> [[i32; WIDTH]; HEIGHT] {
    let mut grid = [[0i32; WIDTH]; HEIGHT];
    for y in 1..=HEIGHT {
        for x in 1..=WIDTH {
            let power = {
                let x = x as i32;
                let y = y as i32;

                let rack_id = x + 10;
                let power = rack_id * y;
                let power = power + SERIAL_NUMBER;
                let power = power * rack_id;
                let power = (power / 100) % 10;
                let power = power - 5;
                power
            };

            grid[y - 1][x - 1] = power;
        }
    }
    grid
}

fn square_sum(grid: &[[i32; WIDTH]; HEIGHT]) -> [[i32; WIDTH - 2]; HEIGHT - 2] {
    let mut sums = [[0i32; WIDTH - 2]; HEIGHT - 2];
    for y in 0..HEIGHT - 2 {
        for x in 0..WIDTH - 2 {
            let sum = grid[y][x]
                + grid[y + 1][x]
                + grid[y + 2][x]
                + grid[y][x + 1]
                + grid[y + 1][x + 1]
                + grid[y + 2][x + 1]
                + grid[y][x + 2]
                + grid[y + 1][x + 2]
                + grid[y + 2][x + 2];
            sums[y][x] = sum;
        }
    }
    sums
}

fn sums_of_all_squares(grid: &[[i32; WIDTH]; HEIGHT]) -> [[(usize, i32); WIDTH]; HEIGHT] {
    let mut sums_all_sqaures = [[(0, 0); WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("Processing cell at {}, {}\r", x, y);

            let max_size = HEIGHT.max(WIDTH) - y.max(x);
            let mut sizes = HashMap::new();
            sizes.insert(1usize, grid[y][x]);

            for s in 1..max_size {
                let mut pwr = *sizes.get(&s).unwrap();
                pwr += grid[y + s][x + s];
                for i in 0..s {
                    pwr += grid[y + s][x + i];
                }
                for i in 0..s {
                    pwr += grid[y + i][x + s];
                }
                sizes.insert(s + 1, pwr);
            }

            let max_of_squares = sizes.into_iter().max_by_key(|(_, pwr)| *pwr).unwrap();
            sums_all_sqaures[y][x] = max_of_squares;
        }
    }

    println!();

    sums_all_sqaures
}
