const NB_GIFTS: usize = 36000000;

fn main() {
    let goal = NB_GIFTS / 10;
    let max: usize = goal - 1;

    println!("Part One:");
    println!("Lowest house number: {}", part_one(goal, max));
    println!("Part Two:");
    println!("Lowest house number: {}", part_two(NB_GIFTS, max));
}

fn part_one(goal: usize, max: usize) -> usize {
    let mut houses = Vec::new();
    houses.resize(max, 1);

    for elf_num in 2..max {
        for house_num in (elf_num..max).step_by(elf_num) {
            houses[house_num] += elf_num;
        }
    }

    for (house_num, house) in houses.iter().enumerate().take(max).skip(2) {
        if *house >= goal {
            return house_num;
        }
    }
    unreachable!()
}

fn part_two(goal: usize, max: usize) -> usize {
    let mut houses = Vec::new();
    houses.resize(max, 1);

    for elf_num in 2..max {
        for house_num in (elf_num..max).step_by(elf_num).take(50) {
            houses[house_num] += elf_num * 11;
        }
    }
    for (house_num, house) in houses.iter().enumerate().take(max).skip(2) {
        if *house >= goal {
            return house_num;
        }
    }
    unreachable!()
}
