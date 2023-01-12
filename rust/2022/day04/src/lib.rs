pub mod input;

pub fn fully_overlapped_pairs(lines: impl Iterator<Item=impl AsRef<str>>) -> usize {
    let mut fully_contained_pairs = 0;
    for line in lines {
        let line = line.as_ref().trim();

        let (first_pair_start,
            first_pair_end,
            second_pair_start,
            second_pair_end) = parse_ranges(line).expect("Malformed line doesn't contain comma (,) separator");
        if range_fully_overlaps(first_pair_start, first_pair_end, second_pair_start, second_pair_end) {
            fully_contained_pairs += 1;
        }
    }
    fully_contained_pairs
}

pub fn partially_overlapped_pairs(lines: impl Iterator<Item=impl AsRef<str>>) -> usize {
    let mut partially_overlapped_pairs = 0;
    for line in lines {
        let line = line.as_ref().trim();

        let (first_pair_start,
            first_pair_end,
            second_pair_start,
            second_pair_end) = parse_ranges(line).expect("Malformed line doesn't contain comma (,) separator");
        if range_partially_overlaps(first_pair_start, first_pair_end, second_pair_start, second_pair_end) {
            partially_overlapped_pairs += 1;
        }
    }
    partially_overlapped_pairs
}

fn parse_ranges(line: &str) -> Option<(usize, usize, usize, usize)> {
    if let Some((first_pair, second_pair)) = line.split_once(',') {
        let (first_pair_start, first_pair_end) = first_pair.split_once('-')
            .expect("First range should contain a dash (-) separator.");
        let first_pair_start: usize = first_pair_start.parse().expect("First pair start should be numeric.");
        let first_pair_end: usize = first_pair_end.parse().expect("First pair end should be numeric.");

        let (second_pair_start, second_pair_end) = second_pair.split_once('-')
            .expect("Range should contain a dash (-) separator.");
        let second_pair_start: usize = second_pair_start.parse().expect("Second pair start should be numeric.");
        let second_pair_end: usize = second_pair_end.parse().expect("Second pair end should be numeric.");

        return Some((first_pair_start, first_pair_end, second_pair_start, second_pair_end));
    }
    None
}

fn range_fully_overlaps(first_pair_start: usize, first_pair_end: usize, second_pair_start: usize, second_pair_end: usize) -> bool {
    if first_pair_end - first_pair_start > second_pair_end - second_pair_start {
        // larger pair is first pair
        first_pair_start <= second_pair_start && first_pair_end >= second_pair_end
    } else {
        // larger pair is second pair
        second_pair_start <= first_pair_start && second_pair_end >= first_pair_end
    }
}

fn range_partially_overlaps(first_pair_start: usize, first_pair_end: usize, second_pair_start: usize, second_pair_end: usize) -> bool {
    if first_pair_start <  second_pair_start {
        // larger pair is first pair
        first_pair_end >= second_pair_start
    } else {
        // larger pair is second pair
        second_pair_end >= first_pair_start
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fully_overlapped_pairs() {
        assert_eq!(fully_overlapped_pairs("\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".lines()), 2);
    }

    #[test]
    fn test_partially_overlapped_pairs() {
        assert_eq!(partially_overlapped_pairs("\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".lines()), 4);
    }
}