use std::collections::HashSet;

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|sack| {
            let (a, b) = sack.split_at(sack.len() / 2);
            let first = a.chars().collect::<HashSet<_>>();
            let second = b.chars().collect::<HashSet<_>>();

            first.intersection(&second).map(|&item| {
                let x = item as u32;
                if x < 91 {
                    x - 38
                } else {
                    x - 96
                }
            }).sum::<u32>()
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|group| {
            let badge = group[0]
                .chars()
                .find(|&item| group[1].contains(item) && group[2].contains(item))
                .unwrap();
            let x = badge as u8;
            if x < 91 {
                (x - 38) as u32
            } else {
                (x - 96) as u32
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let result = 157;
        assert_eq!(result, part_one(input));
    }

    #[test]
    fn part_two_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let result = 70;
        assert_eq!(result, part_two(input));
    }
}
