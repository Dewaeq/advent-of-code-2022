pub mod method_one {
    pub fn part_one(input: &str) -> u32 {
        // A & X = rock
        // B & Y = paper
        // C & Z = scissors

        input
            .lines()
            .map(|round| match round {
                // opp plays rock
                "A X" => (1 + 3),
                "A Y" => (2 + 6),
                "A Z" => (3 + 0),
                // opp plays paper
                "B X" => (1 + 0),
                "B Y" => (2 + 3),
                "B Z" => (3 + 6),
                // opp plays scissors
                "C X" => (1 + 6),
                "C Y" => (2 + 0),
                "C Z" => (3 + 3),
                _ => 0,
            })
            .sum::<u32>()
    }

    pub fn part_two(input: &str) -> u32 {
        // X = lose
        // Y = draw
        // Z = win

        input
            .lines()
            .map(|round| match round {
                // opp plays rock
                "A X" => (3 + 0),
                "A Y" => (1 + 3),
                "A Z" => (2 + 6),
                // opp plays paper
                "B X" => (1 + 0),
                "B Y" => (2 + 3),
                "B Z" => (3 + 6),
                // opp plays scissors
                "C X" => (2 + 0),
                "C Y" => (3 + 3),
                "C Z" => (1 + 6),
                _ => 0,
            })
            .sum::<u32>()
    }
}

pub mod method_two {
    fn parse_input(input: &str) -> Vec<String> {
        let parse = |f: char| match f {
            'A' | 'X' => "R",
            'B' | 'Y' => "P",
            'C' | 'Z' => "S",
            _ => "",
        };

        let mut output = vec![];

        input.lines().for_each(|round| {
            let chars = round.chars().collect::<Vec<_>>();
            let m1 = parse(*chars.get(0).unwrap());
            let m2 = parse(*chars.get(2).unwrap());

            output.push(format!("{}{}", m1, m2));
        });

        output
    }

    fn score(round: &str) -> u32 {
        let outcome = match round {
            "SR" | "RP" | "PS" => 6,
            "RR" | "PP" | "SS" => 3,
            _ => 0,
        };

        let shape = match round.chars().nth(1).unwrap() {
            'R' => 1,
            'P' => 2,
            _ => 3,
        };

        outcome + shape
    }

    pub fn part_one(input: &str) -> u32 {
        let input = parse_input(input);
        input.iter().map(|round| score(round)).sum::<u32>()
    }

    pub fn part_two(input: &str) -> u32 {
        let input = parse_input(input);
        input
            .iter()
            .map(|round| {
                score(match round.as_str() {
                    "RR" => "RS",
                    "RP" => "RR",
                    "RS" => "RP",
                    "PR" => "PR",
                    "PP" => "PP",
                    "PS" => "PS",
                    "SR" => "SP",
                    "SP" => "SS",
                    "SS" => "SR",
                    _ => "",
                })
            })
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = "A Y
B X
C Z";
        let result = 15;
        assert_eq!(result, method_one::part_one(input));
        assert_eq!(result, method_two::part_one(input));
    }

    #[test]
    fn part_two_test() {
        let input = "A Y
B X
C Z";
        let result = 12;
        assert_eq!(result, method_one::part_two(input));
        assert_eq!(result, method_two::part_two(input));
    }
}
