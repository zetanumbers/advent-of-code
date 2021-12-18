const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_18::part_1(INPUT), 3524);
}

#[test]
fn part_2() {
    assert_eq!(day_18::part_2(INPUT), 4656);
}
