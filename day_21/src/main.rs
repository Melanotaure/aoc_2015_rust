use std::fs::read_to_string;

struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

struct Shop {
    weapons: Vec<Item>,
    armors: Vec<Item>,
    rings: Vec<Item>,
}

#[derive(Debug, Clone)]
struct Attacker {
    hps: u32,
    damage: u32,
    armor: u32,
}

fn main() {
    let filename = String::from("puzzle_input.txt");
    let input = read_to_string(&filename).expect("Unable to read puzzle file");

    let mut lines = input.lines();

    let hps: u32 = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .strip_prefix(' ')
        .unwrap()
        .parse()
        .unwrap();
    let damage: u32 = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .strip_prefix(' ')
        .unwrap()
        .parse()
        .unwrap();
    let armor: u32 = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .strip_prefix(' ')
        .unwrap()
        .parse()
        .unwrap();

    let boss = Attacker { hps, damage, armor };

    println!("boss: {:?}", boss);

    let mut shop = Shop {
        weapons: Vec::<Item>::new(),
        armors: Vec::<Item>::new(),
        rings: Vec::<Item>::new(),
    };
    // weapons
    let dagger = Item {
        cost: 8,
        damage: 4,
        armor: 0,
    };
    shop.weapons.push(dagger);
    let shortsword = Item {
        cost: 10,
        damage: 5,
        armor: 0,
    };
    shop.weapons.push(shortsword);
    let warhammer = Item {
        cost: 25,
        damage: 6,
        armor: 0,
    };
    shop.weapons.push(warhammer);
    let longsword = Item {
        cost: 40,
        damage: 7,
        armor: 0,
    };
    shop.weapons.push(longsword);
    let greataxe = Item {
        cost: 74,
        damage: 8,
        armor: 0,
    };
    shop.weapons.push(greataxe);
    // Armors
    let no_armor = Item {
        cost: 0,
        damage: 0,
        armor: 0,
    };
    shop.armors.push(no_armor);
    let leather = Item {
        cost: 13,
        damage: 0,
        armor: 1,
    };
    shop.armors.push(leather);
    let chainmail = Item {
        cost: 31,
        damage: 0,
        armor: 2,
    };
    shop.armors.push(chainmail);
    let splintmail = Item {
        cost: 53,
        damage: 0,
        armor: 3,
    };
    shop.armors.push(splintmail);
    let bandedmail = Item {
        cost: 75,
        damage: 0,
        armor: 4,
    };
    shop.armors.push(bandedmail);
    let platemail = Item {
        cost: 102,
        damage: 0,
        armor: 5,
    };
    shop.armors.push(platemail);
    // rings
    let no_ring = Item {
        cost: 0,
        damage: 0,
        armor: 0,
    };
    shop.rings.push(no_ring);
    let dmg1 = Item {
        cost: 25,
        damage: 1,
        armor: 0,
    };
    shop.rings.push(dmg1);
    let dmg2 = Item {
        cost: 50,
        damage: 2,
        armor: 0,
    };
    shop.rings.push(dmg2);
    let dmg3 = Item {
        cost: 100,
        damage: 3,
        armor: 0,
    };
    shop.rings.push(dmg3);
    let def1 = Item {
        cost: 20,
        damage: 0,
        armor: 1,
    };
    shop.rings.push(def1);
    let def2 = Item {
        cost: 40,
        damage: 0,
        armor: 2,
    };
    shop.rings.push(def2);
    let def3 = Item {
        cost: 80,
        damage: 0,
        armor: 3,
    };
    shop.rings.push(def3);

    let (part_one, part_two) = min_and_max_gold_spent_to_win(&shop, &boss);
    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);
}

fn min_and_max_gold_spent_to_win(shop: &Shop, boss_original: &Attacker) -> (u32, u32) {
    let mut min_spent = std::u32::MAX;
    let mut max_spent = 0;
    let mut player;
    let mut boss = boss_original.clone();

    for weapon in shop.weapons.iter() {
        for armor in shop.armors.iter() {
            for (ring1_index, ring1) in shop.rings[0..shop.rings.len() - 1].iter().enumerate() {
                // We can’t have two times the same item, so the 2nd ring is always after the 1st,
                // except if ring1_index is 0, because we can have two times “No ring”
                let ring2_index_start = if ring1_index == 0 { 0 } else { ring1_index + 1 };
                for ring2 in shop.rings[ring2_index_start..].iter() {
                    player = Attacker {
                        hps: 100,
                        damage: weapon.damage + ring1.damage + ring2.damage,
                        armor: armor.armor + ring1.armor + ring2.armor,
                    };
                    boss.hps = boss_original.hps;
                    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                    if player_wins(&mut player, &mut boss) {
                        min_spent = std::cmp::min(min_spent, cost);
                    } else {
                        max_spent = std::cmp::max(max_spent, cost);
                    }
                }
            }
        }
    }

    (min_spent, max_spent)
}

fn player_wins(player: &mut Attacker, boss: &mut Attacker) -> bool {
    let player_real_damage = if player.damage > boss.armor {
        player.damage - boss.armor
    } else {
        1
    };
    let boss_real_damage = if boss.damage > player.armor {
        boss.damage - player.armor
    } else {
        1
    };

    loop {
        if player_real_damage >= boss.hps {
            return true;
        }
        boss.hps -= player_real_damage;
        if boss_real_damage >= player.hps {
            return false;
        }
        player.hps -= boss_real_damage;
    }
}
