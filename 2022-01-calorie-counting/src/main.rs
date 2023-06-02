use std::fs;

fn main() {
    let inventories = fs::read_to_string("input.txt").expect("should have existed");

    let mut elves: Vec<Elf> = inventories
        .split("\n\n")
        .into_iter()
        .enumerate()
        .map(|(i, inventory)| Elf::new(i, inventory))
        .collect();

    elves.sort_by_key(|x| u32::MAX - x.calories);

    println!(
        "Part 1: Elf {} is carrying the most calories, at {} kcal.",
        elves[0].number, elves[0].calories
    );

    println!(
        "Part 2: Elves {}, {}, and {} are carrying a total of {} kcal.",
        elves[0].number,
        elves[1].number,
        elves[2].number,
        elves[0].calories + elves[1].calories + elves[2].calories
    );
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    pub number: usize,
    pub calories: u32,
}

impl Elf {
    pub fn new(index: usize, inventory: &str) -> Self {
        Elf {
            calories: inventory
                .split('\n')
                .filter(|x| 0 < x.len())
                .map(|x| x.parse::<u32>().expect("should have been an integer"))
                .reduce(|x, y| x + y)
                .unwrap(),
            number: index + 1,
        }
    }
}
