use aoc_2022_01::{part1, part2};

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 69883);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 195625);
}
