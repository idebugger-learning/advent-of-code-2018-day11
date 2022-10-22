const WIDTH: usize = 300;
const HEIGHT: usize = 300;
// const SERIAL_NUMBER: i32 = 18;
const SERIAL_NUMBER: i32 = 7403;

fn main() {
    let grid = init_grid();
    let sums = square_sum(&grid);

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
