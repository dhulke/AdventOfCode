pub mod input;

pub fn highest_group_calories(calory_groups: impl Iterator<Item=impl AsRef<str>>) -> usize {
    let mut highest_group_calories = 0;
    let mut current_group_calories = 0;
    for line in calory_groups {
        let line = line.as_ref().trim();
        if line.is_empty() {
            if current_group_calories > highest_group_calories {
                highest_group_calories = current_group_calories;
            }
            current_group_calories = 0;
        } else {
            current_group_calories += line.parse::<usize>().expect("Malformed file. Expected only number and empty lines");
        }
    }
    if current_group_calories > highest_group_calories {
        highest_group_calories = current_group_calories;
    }
    highest_group_calories
}

pub fn top_n_highest_group_calories(calory_groups: impl Iterator<Item=impl AsRef<str>>, n: usize) -> usize {
    let mut current_group_calories = 0;
    let mut sum_calory_groups: Vec<usize> = vec![];
    for line in calory_groups {
        let line = line.as_ref().trim();
        if line.is_empty() {
            sum_calory_groups.push(current_group_calories);
            current_group_calories = 0;
        } else {
            current_group_calories += line.parse::<usize>().expect("Malformed file. Expected only number and empty lines");
        }
    }
    sum_calory_groups.push(current_group_calories);
    sum_calory_groups.sort_unstable();

    let start_index = sum_calory_groups.len().checked_sub(n).unwrap_or(0);

    sum_calory_groups
        .get(start_index..sum_calory_groups.len())
        .expect("Range is always valid: at least 0 and at most len()")
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_calories_no_groups() {
        assert_eq!(highest_group_calories("".lines()), 0);
    }

    #[test]
    fn test_highest_calories_one_group() {
        assert_eq!(highest_group_calories("\
1000
2000
3000

".lines()), 6000);
    }

    #[test]
    fn test_highest_calories_two_groups() {
        assert_eq!(highest_group_calories("\
1000
2000
3000

4000
5000
6000".lines()), 15000);
    }

    #[test]
    fn test_highest_calories_three_groups() {
        assert_eq!(highest_group_calories("\
1000
2000
3000

4000
5000
6000

7000
8000
9000".lines()), 24000);
    }

    #[test]
    fn test_top_three_highest_calories_no_groups() {
        assert_eq!(top_n_highest_group_calories("".lines(), 3), 0);
    }

    #[test]
    fn test_top_three_highest_calories_one_group() {
        assert_eq!(top_n_highest_group_calories("\
1000
2000
3000

".lines(), 3), 6000);
    }

    #[test]
    fn test_top_three_highest_calories_two_groups() {
        assert_eq!(top_n_highest_group_calories("\
1000
2000
3000

4000
5000
6000".lines(), 3), 21000);
    }

    #[test]
    fn test_top_three_highest_calories_three_groups() {
        assert_eq!(top_n_highest_group_calories("\
1000
2000
3000

4000
5000
6000

7000
8000
9000".lines(), 3), 45000);
    }

    #[test]
    fn test_top_three_highest_calories_five_groups() {
        assert_eq!(top_n_highest_group_calories("\
500
500

2000

0

4000

5000".lines(), 3), 11000);
    }
}
