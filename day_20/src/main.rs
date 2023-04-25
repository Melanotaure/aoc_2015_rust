const NB_GIFTS: u32 = 36000000;

fn main() {
    let goal = (NB_GIFTS / 10) as usize;
    let max: usize = goal - 1;
    let mut houses = Vec::new();
    houses.resize(max, 1);

    for elf_num in 2..max {
        for house_num in (elf_num..max).step_by(elf_num) {
            houses[house_num] += elf_num;
        }
    }

    let mut saved_house_num = 0;
    for (house_num, house) in houses.iter().enumerate().take(max).skip(2) {
        if *house >= goal {
            saved_house_num = house_num;
            break;
        }
    }

    println!("Part One:");
    println!("Lowest house number: {}", saved_house_num);
}
