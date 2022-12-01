use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    let mut input_file = OpenOptions::new()
        .read(true)
        .open("./input/input.txt")
        .unwrap();

    let mut input_buffer = String::new();

    input_file.read_to_string(&mut input_buffer).unwrap();

    let iter = input_buffer.split("\n\n");

    let mut totals = Vec::new();

    for (_i, elf) in iter.enumerate() {
        let iter_elf = elf.split("\n");

        let sum = iter_elf.fold(0, |sum, x| sum + x.parse::<u32>().unwrap());

        totals.push(sum);
    }

    totals.sort();

    println!("Most with 1 elf: {:?}", totals.last().unwrap());

    println!("Total of top 3 elves: {:?}", &totals[totals.len() - 3..].iter().sum::<u32>());

}
