use std::fs::read_to_string;

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file.");

    let grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();

    println!("Part One:");
    println!("Number of lights on: {}", count_lights_on(&grid, false));
    println!("Part Two: [when corners lights are on]");
    println!("Number of lights on: {}", count_lights_on(&grid, true));
}

fn count_lights_on(grid: &Vec<Vec<bool>>, corners_on: bool) -> u32 {
    let mut grid = grid.clone();
    for _ in 0..100 {
        let old_grid = grid.clone();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let count = neighbors_count(&old_grid, x, y, corners_on);
                grid[x][y] = count == 3 || (old_grid[x][y] && count == 2);
            }
        }
    }

    grid.iter()
        .flat_map(|l| l.iter())
        .fold(0, |acc, x| if *x { acc + 1 } else { acc })
}

fn neighbors_count(grid: &Vec<Vec<bool>>, x: usize, y: usize, corners_on: bool) -> u32 {
    let has_left = x > 0;
    let has_right = x < grid.len() - 1;
    let has_up = y > 0;
    let has_down = y < grid[0].len() - 1;

    if corners_on && (!has_left || !has_right) && (!has_up || !has_down) {
        return 3;
    }

    let mut neighbors = 0;

    if has_left && grid[x - 1][y] {
        neighbors += 1;
    }
    if has_up && grid[x][y - 1] {
        neighbors += 1;
    }
    if has_right && grid[x + 1][y] {
        neighbors += 1;
    }
    if has_down && grid[x][y + 1] {
        neighbors += 1;
    }

    if has_left && has_up && grid[x - 1][y - 1] {
        neighbors += 1;
    }
    if has_right && has_up && grid[x + 1][y - 1] {
        neighbors += 1;
    }
    if has_right && has_down && grid[x + 1][y + 1] {
        neighbors += 1;
    }
    if has_left && has_down && grid[x - 1][y + 1] {
        neighbors += 1;
    }

    neighbors
}
