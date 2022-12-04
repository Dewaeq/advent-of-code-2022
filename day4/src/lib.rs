struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn from_str(s: &str) -> Self {
        let (start, end) = s.split_once("-").unwrap();
        let start = start.parse::<u32>().unwrap();
        let end = end.parse::<u32>().unwrap();

        Section { start, end }
    }

    fn contains(&self, other: &Section) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Section) -> bool {
        self.end >= other.start && self.start <= other.end
    }
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .filter(|pairs| {
            let (first, second) = pairs.split_once(",").unwrap();
            let a = Section::from_str(first);
            let b = Section::from_str(second);

            a.contains(&b) || b.contains(&a)
        })
        .count() as u32
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .filter(|pairs| {
            let (first, second) = pairs.split_once(",").unwrap();
            let a = Section::from_str(first);
            let b = Section::from_str(second);

            a.overlaps(&b) || b.overlaps(&a)
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = 2;
        assert_eq!(result, part_one(input));
    }

    #[test]
    fn part_two_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = 4;
        assert_eq!(result, part_two(input));
    }
}
