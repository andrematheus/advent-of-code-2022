use std::cmp::Reverse;

fn total_calories_of_elf_carrying_the_most(input: &str) -> i64 {
    let calories_for_each_elf = sorted_calories_for_each_elf(input);
    *calories_for_each_elf.first().expect("Empty vector")
}

fn total_calories_of_top_three_elves(input: &str) -> i64 {
    let calories_for_each_elf = sorted_calories_for_each_elf(input);
    calories_for_each_elf.iter().take(3).sum()
}

fn sorted_calories_for_each_elf(input: &str) -> Vec<i64> {
    let mut calories_for_each_elf: Vec<i64> = input.split('\n')
        .fold(vec![0], |mut acc: Vec<i64>, item: &str| {
            if item.is_empty() {
                acc.push(0);
            } else {
                let item: i64 = item.parse().expect("Invalid input");
                let last = acc.last_mut().unwrap();
                *last += item;
            }
            acc
        });
    calories_for_each_elf.sort_by_key(|i| Reverse(*i));
    calories_for_each_elf
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_example() {
        let result = total_calories_of_elf_carrying_the_most(EXAMPLE_INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part2_example() {
        let result = total_calories_of_top_three_elves(EXAMPLE_INPUT);
        assert_eq!(result, 45000);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", total_calories_of_elf_carrying_the_most(input));
    println!("{}", total_calories_of_top_three_elves(input));
}