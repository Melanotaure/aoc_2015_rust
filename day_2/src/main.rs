use std::fs::read_to_string;

struct Box {
    l: u32,
    w: u32,
    h: u32,
}

impl Box {
    fn new(l: u32, w: u32, h: u32) -> Self {
        Self { l: l, w: w, h: h }
    }

    fn total_surface(&self) -> u32 {
        2 * (self.l * self.w + self.w * self.h + self.h * self.l)
    }

    fn smallest_surface(&self) -> u32 {
        let mut surf = vec![self.l, self.w, self.h];
        surf.sort();
        surf[0] * surf[1]
    }
}

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file");

    let mut total = 0;
    for line in input.lines() {
        let dim: Vec<&str> = line.split('x').collect();
        let tmp_box = Box::new(
            dim[0].parse().unwrap(),
            dim[1].parse().unwrap(),
            dim[2].parse().unwrap(),
        );
        total += tmp_box.total_surface() + tmp_box.smallest_surface();
    }

    println!("Part One:");
    println!("  Total wrapping paper surface: {} square feet.", total);
}
