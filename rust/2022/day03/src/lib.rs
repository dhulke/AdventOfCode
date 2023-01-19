use std::collections::HashSet;

pub mod input;

/// Response to the first part
pub fn rucksacks_priorities_sum(lines: impl Iterator<Item=impl AsRef<str>>) -> usize {
    let mut rucksacks_priorities_sum = 0;
    for line in lines {
        let line = line.as_ref().trim();
        rucksacks_priorities_sum += Rucksack::new(line.to_string()).shared_priorities_sum()
    }
    rucksacks_priorities_sum
}

/// Response to the second part
pub fn rucksacks_group_badges_sum(lines: impl Iterator<Item=impl AsRef<str>>) -> usize {
    let mut rucksacks_group_badges_sum = 0;
    let mut lines = lines.into_iter();
    while let Some(first_rucksack) = lines.next() {
        let first_rucksack = Rucksack::new(first_rucksack.as_ref().to_string());
        let second_rucksack = Rucksack::new(
            lines
                .next()
                .expect("The number of rucksacks should always be divisible by 3.")
                .as_ref()
                .to_string());
        let third_rucksack = Rucksack::new(
            lines
                .next()
                .expect("The number of rucksacks should always be divisible by 3.")
                .as_ref()
                .to_string());

        rucksacks_group_badges_sum += get_priority(&get_item_intersection_in_rucksacks(
            first_rucksack,
            second_rucksack,
            third_rucksack));
    }
    rucksacks_group_badges_sum
}

struct Rucksack {
    items: String,
}

impl Rucksack {

    fn new(items: String) -> Self {
        if items.len() % 2 != 0 {
            panic!("There should always be an even number of items.");
        }
        for item in items.chars() {
            if !item.is_alphabetic() {
                panic!("There should always be only alphabetic items.");
            }
        }
        Self {items}
    }

    fn shared_priorities_sum(&self) -> usize {
        let mid_point = self.items.len() / 2;
        let first_compartment = get_set_with_chars(&self.items[..mid_point]);
        let second_compartment = get_set_with_chars(&self.items[mid_point..]);
        let mut priorities_sum = 0;

        for item in &first_compartment {
            if second_compartment.contains(&item) {
                priorities_sum += get_priority(&item);
            }
        }
        priorities_sum
    }

    fn get_set(&self) -> HashSet<char> {
        get_set_with_chars(&self.items)
    }

}

fn get_priority(item: &char) -> usize {
    let item = *item as u32;
    (if  item >= ('a' as u32) && item <= ('z' as u32) {
        item - ('a' as u32) + 1
    } else {
        // we don't need to check uppercase range because new() guarantees we have only alphabetic characters
        item - ('A' as u32) + 27
    }) as usize
}

fn get_set_with_chars(items: &str) -> HashSet<char> {
    items.chars().collect()
}

fn get_item_intersection_in_rucksacks(first: Rucksack, second: Rucksack, third: Rucksack) -> char {
    let intersection_set =
        &(&first.get_set() & &second.get_set())
        & &third.get_set();
    intersection_set.into_iter().next().expect("There should be at least one intersected item.")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_rucksack_new_odd_number_of_items() {
        Rucksack::new("abc".to_string());
    }

    #[test]
    #[should_panic]
    fn test_rucksack_new_non_alphabetic_items() {
        Rucksack::new("ab1c".to_string());
    }

    #[test]
    fn test_rucksack_shared_common_priority() {
        assert_eq!(Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp".to_string()).shared_priorities_sum(), 16);
        assert_eq!(Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()).shared_priorities_sum(), 38);
        assert_eq!(Rucksack::new("PmmdzqPrVvPwwTWBwg".to_string()).shared_priorities_sum(), 42);
        assert_eq!(Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string()).shared_priorities_sum(), 22);
        assert_eq!(Rucksack::new("ttgJtRGJQctTZtZT".to_string()).shared_priorities_sum(), 20);
        assert_eq!(Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw".to_string()).shared_priorities_sum(), 19);
    }

    #[test]
    fn test_rucksacks_priorities_sum() {
        assert_eq!(rucksacks_priorities_sum("\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".lines()), 157);
    }

    #[test]
    fn test_rucksacks_group_badges_sum() {
        assert_eq!(rucksacks_group_badges_sum("\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".lines()), 70);
    }
}