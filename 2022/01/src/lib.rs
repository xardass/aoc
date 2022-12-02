pub fn part1(input: &str) -> u32 {
    let iter = input.split("\n\n");
    let mut totals = Vec::new();
    for (_i, elf) in iter.enumerate() {
        let iter_elf = elf.split("\n");

        let sum = iter_elf.fold(0, |sum, x| sum + x.parse::<u32>().unwrap());

        totals.push(sum);
    }
    totals.sort();
    *totals.last().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let iter = input.split("\n\n");
    let mut totals = Vec::new();
    for (_i, elf) in iter.enumerate() {
        let iter_elf = elf.split("\n");

        let sum = iter_elf.fold(0, |sum, x| sum + x.parse::<u32>().unwrap());

        totals.push(sum);
    }
    totals.sort();
    totals[totals.len() - 3..].iter().sum::<u32>()
}
