use std::cell::RefCell;
use std::collections::HashMap;

pub mod input;

type CrateStacksMap = HashMap<String, Vec<String>>;

/// We store ordered_stack_names in order to create crate_stacks in the order it was inserted
#[derive(PartialEq, Debug)]
pub struct CrateStacks {
    ordered_stack_names: Vec<String>,
    crate_stacks: CrateStacksMap,
}

impl CrateStacks {
    fn new(ordered_stack_names: Vec<String>, crate_stacks: CrateStacksMap) -> Self {
        Self { ordered_stack_names, crate_stacks }
    }

    /// Doesn't do anything if stack_index doesn't exist
    fn add_to_stack(&mut self, stack_index: usize, crate_name: String) {
        if let Some(stack_name) = self.ordered_stack_names.get(stack_index) {
            if let Some(stack) = self.crate_stacks.get_mut(stack_name) {
                stack.push(crate_name);
            }
        }
    }

    fn move_many_from_top(&mut self, n: usize, from_stack: &str, to_stack: &str) {
        for _ in 0..n {
            self.move_from_to_stack(from_stack, to_stack);
        }
    }

    /// Doesn't do anything if trying to move from a stack without crates
    fn move_many_from_bottom(&mut self, n: usize, from_stack: &str, to_stack: &str) {
        let mut crates_being_moved = None;
        if let Some(from_stack) = self.crate_stacks.get_mut(from_stack) {
            let n = if from_stack.len() < n { from_stack.len() } else { n }; // Don't allow n to move more crates there there are
            crates_being_moved = Some(from_stack.drain((from_stack.len() - n)..).collect::<Vec<String>>());
        }

        if let Some(to_stack) = self.crate_stacks.get_mut(to_stack) {
            to_stack.extend(crates_being_moved.expect("Guaranteed to have some at this point"));
        }
    }

    /**
        Move one crate from the top of <from_stack> to the top of <to_stack>.
        Doesn't do anything if trying to move from a stack without crates
    */
    fn move_from_to_stack(&mut self, from_stack: &str, to_stack: &str) {
        let mut crate_name_being_moved = None;
        if let Some(from_stack) = self.crate_stacks.get_mut(from_stack) {
            if let Some(crate_name) = from_stack.pop() {
                crate_name_being_moved = Some(crate_name)
            } else {
                return;
            }
        }

        if let Some(to_stack) = self.crate_stacks.get_mut(to_stack) {
            to_stack.push(crate_name_being_moved.expect("Crate name exists at this point"));
        }
    }

    /// Returns top crates from stacks as string. If a stack doesn't have any crates, return a blank space.
    fn get_top_crates(&self) -> String {
        self.ordered_stack_names
            .iter()
            .map(|crate_name|
                self.crate_stacks
                    .get(crate_name)
                    .expect(&format!("Expect there to be a stack for crate: {}", crate_name))
                    .last()
                    .map(String::as_str))
            .map(|a| if a.is_none() { Some(" ") } else { a }) // problem statement didn't mention this case
            .map(|crate_name| crate_name.expect("There should always be Some() at this point"))
            .collect::<Vec<&str>>()
            .join("")
    }
}

/**
This is a factory module that initializes CrateStacks from Iterator<Item=String>. We could have
other factory modules that initialize CrateStacks from different sources.
 */
pub mod crate_stacks_lines_parser {
    use super::*;

    struct MoveInstruction {
        n: usize,
        from_stack: String,
        to_stack: String
    }

    /**
    Read the lines iterator until we reach the end of crate stacks (blank line) storing these
    lines in a Vec for creating the CrateStacks struct. Then pass the remaining iterator to
    get_iterator_with_move_instructions that will parse the instructions that will mutate CrateStacks.
     */
    pub fn parse_from_top_all_instructions_from_lines(mut lines: impl Iterator<Item=String>) -> CrateStacks {
        let mut crate_stacks = parse_crate_stacks(&mut lines);
        get_iterator_with_move_instructions(&mut lines)
            .for_each(|move_instruction|
                crate_stacks.move_many_from_top(move_instruction.n,
                                                &move_instruction.from_stack,
                                                &move_instruction.to_stack));
        crate_stacks
    }

    /**
    Read the lines iterator until we reach the end of crate stacks (blank line) storing these
    lines in a Vec for creating the CrateStacks struct. Then pass the remaining iterator to
    get_iterator_with_move_instructions that will parse the instructions that will mutate CrateStacks.
     */
    pub fn parse_from_bottom_all_instructions_from_lines(mut lines: impl Iterator<Item=String>) -> CrateStacks {
        let mut crate_stacks = parse_crate_stacks(&mut lines);
        get_iterator_with_move_instructions(&mut lines)
            .for_each(|move_instruction|
                crate_stacks.move_many_from_bottom(move_instruction.n,
                                                   &move_instruction.from_stack,
                                                   &move_instruction.to_stack));
        crate_stacks
    }

    /**
        Read the lines iterator until we reach the end of crate stacks (blank line) storing these
        lines in a Vec for creating the CrateStacks struct.
    */
    fn parse_crate_stacks(lines: &mut impl Iterator<Item=String>) -> CrateStacks {
        let mut crate_stacks_lines = vec![];
        for line in lines {
            if line.trim().is_empty() {
                // Stop the iterator to allow parse_move_instructions_from_lines() to continue with it
                break;
            }
            crate_stacks_lines.push(line.to_string());
        }
        let mut crate_stacks = new_crate_stacks(
            crate_stacks_lines.pop().expect("Expect there to be a line with crate stacks names"));
        populate_crate_stacks(&mut crate_stacks, crate_stacks_lines);
        crate_stacks
    }

    /// Parse move instructions with the format: move <n> from <from_stack> to <to_stack>
    fn get_iterator_with_move_instructions<'a>(lines: &'a mut impl Iterator<Item=String>) -> impl Iterator<Item=MoveInstruction> + 'a {
        lines.map(|line| {
            let mut parts = line.trim().split(' ');
            MoveInstruction {
                n: parts.nth(1).expect("Expect to have number of crates").parse().expect("Expect to be usize"), // jump 2
                from_stack: parts.nth(1).expect("Expect to have from stack name").to_string(), // jump 2
                to_stack: parts.nth(1).expect("Expect to have stack name").to_string()
            }
        })
    }

    /// Creates CrateStacks but initializing all stacks with an empty Vec<String>
    fn new_crate_stacks<'a>(line: String) -> CrateStacks {
        let (ordered_stack_names, crate_stacks): (Vec<String>, CrateStacksMap) = line
            .trim()
            .split("   ")
            .map(|name| (name.to_string(), (name.to_string(), vec![])))
            .unzip();

        CrateStacks::new(ordered_stack_names, crate_stacks)
    }

    /// Populate CrateStacks by callins add_to_stack on each index where a crate is found
    fn populate_crate_stacks<'a>(crate_stacks: &mut CrateStacks, lines: Vec<String>) {
        for line in lines.into_iter().rev() {
            (1..line.len())
                .step_by(4)// i += 4
                .map(|i| &line[i..i + 1])// return crate name as a slice
                .enumerate()
                .filter(|(i, crate_name)| !crate_name.trim().is_empty())
                .for_each(|(i, crate_name)| crate_stacks.add_to_stack(i, crate_name.to_string()))
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_parse_from_top_all_instructions_from_lines() {
            assert_eq!(parse_from_top_all_instructions_from_lines("    [b]
    [a] [p]
    [q] [y] [t]
[f] [s] [w] [e]
[a] [b] [c] [d]
 1   2   3   4

move 10 from 1 to 2
move 2 from 4 to 1".lines().map(String::from)),
                       CrateStacks::new(
                           vec!["1", "2", "3", "4"].into_iter().map(String::from).collect(),
                           vec![
                               ("1".to_string(), vec!["t", "e"].into_iter().map(String::from).collect()),
                               ("2".to_string(), vec!["b", "s", "q", "a", "b", "f", "a"].into_iter().map(String::from).collect()),
                               ("3".to_string(), vec!["c", "w", "y", "p"].into_iter().map(String::from).collect()),
                               ("4".to_string(), vec!["d"].into_iter().map(String::from).collect())]
                               .into_iter().collect()));
        }
    }

    #[test]
    fn test_parse_from_bottom_all_instructions_from_lines() {
        assert_eq!(parse_from_bottom_all_instructions_from_lines("    [b]
    [a] [p]
    [q] [y] [t]
[f] [s] [w] [e]
[a] [b] [c] [d]
 1   2   3   4

move 10 from 1 to 2
move 2 from 4 to 1".lines().map(String::from)),
                   CrateStacks::new(
                       vec!["1", "2", "3", "4"].into_iter().map(String::from).collect(),
                       vec![
                           ("1".to_string(), vec!["e", "t"].into_iter().map(String::from).collect()),
                           ("2".to_string(), vec!["b", "s", "q", "a", "b", "a", "f"].into_iter().map(String::from).collect()),
                           ("3".to_string(), vec!["c", "w", "y", "p"].into_iter().map(String::from).collect()),
                           ("4".to_string(), vec!["d"].into_iter().map(String::from).collect())]
                           .into_iter().collect()));
    }
}

pub fn get_top_crates_after_moves_from_top(lines: impl Iterator<Item=String>) -> String {
    crate_stacks_lines_parser::parse_from_top_all_instructions_from_lines(lines).get_top_crates()
}

pub fn get_top_crates_after_moves_from_bottom(lines: impl Iterator<Item=String>) -> String {
    crate_stacks_lines_parser::parse_from_bottom_all_instructions_from_lines(lines).get_top_crates()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_stacks_move_many_from_top_get_top_crates() {
        let mut crate_stacks = CrateStacks::new(
            vec!["1", "2", "3", "4"].into_iter().map(String::from).collect(),
            vec![
                ("1".to_string(), vec!["a", "f", "q"].into_iter().map(String::from).collect()),
                ("2".to_string(), vec!["b", "s"].into_iter().map(String::from).collect()),
                ("3".to_string(), vec!["c"].into_iter().map(String::from).collect()),
                ("4".to_string(), vec!["d", "e", "t"].into_iter().map(String::from).collect())]
                .into_iter().collect());
        crate_stacks.move_many_from_top(10, "1", "2");
        crate_stacks.move_many_from_top(2, "4", "1");
        crate_stacks.move_many_from_top(1, "3", "1");

        assert_eq!(crate_stacks.get_top_crates(), "ca d");
    }

    #[test]
    fn test_create_stacks_move_many_from_bottom_get_top_crates() {
        let mut crate_stacks = CrateStacks::new(
            vec!["1", "2", "3", "4"].into_iter().map(String::from).collect(),
            vec![
                ("1".to_string(), vec!["a", "f", "q"].into_iter().map(String::from).collect()),
                ("2".to_string(), vec!["b", "s"].into_iter().map(String::from).collect()),
                ("3".to_string(), vec!["c"].into_iter().map(String::from).collect()),
                ("4".to_string(), vec!["d", "e", "t"].into_iter().map(String::from).collect())]
                .into_iter().collect());
        crate_stacks.move_many_from_bottom(10, "1", "2");
        crate_stacks.move_many_from_bottom(2, "4", "1");
        crate_stacks.move_many_from_bottom(1, "3", "1");

        assert_eq!(crate_stacks.get_top_crates(), "cq d");
    }
}