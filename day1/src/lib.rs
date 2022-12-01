pub fn part_one(input: &str) -> u32 {
    input
        .replace("\r", "")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| {
                    // println!("{}", line);
                    line.parse::<u32>().expect(line)
                })
                .sum()
        })
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let mut elves = input
        .replace("\r", "")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| {
                    // println!("{}", line);
                    line.parse::<u32>().expect(line)
                })
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    elves.sort_by(|a, b| b.cmp(a));
    elves.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let result: u32 = 24000;
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(result, part_one(input));
    }

    #[test]
    fn part_two_test() {
        let result: u32 = 45000;
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(result, part_two(input));
    }
}
