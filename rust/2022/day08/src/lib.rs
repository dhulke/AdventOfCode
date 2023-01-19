use std::collections::HashSet;

pub mod input;

/// A TreePatch is a square of numbers representing tree heights
type TreePatch = Vec<Vec<u8>>;

/// Structure storing the x/y position of a tree in a TreePatch
#[derive(PartialEq, Eq, Hash)]
pub struct Tree(usize, usize);

impl Tree {
    fn new(x: usize, y: usize) -> Self {
        Self (x, y)
    }
}

/**
    A dedicated module to parsing the input and providing the model for the algorithm to work with.
    There could be other methods in this module and other modules dedicated to generating the same
    model from different sources.
*/
mod tree_patch_parser {
    use super::*;
    pub fn parse_from_text(lines: impl Iterator<Item=String>) -> TreePatch {
        lines
            .map(|line|
                line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect())
            .collect()
    }
}

/// This module groups together all of the functions related to viewing trees in a tree patch.
mod tree_viewer {
    use super::*;

    pub mod outside_in {
        use super::*;

        /**
            Computes all visible trees from the outside. Groups visible trees from each side in a
            hashset to remove duplicates.
        */
        pub fn get_visible_trees_count(tree_patch: &TreePatch) -> usize {
            let mut visible_trees = HashSet::new();

            for y in 0..tree_patch.len() {
                visible_trees.extend(from_left(y, tree_patch));
                visible_trees.extend(from_right(y, tree_patch));
            }
            for x in 0..tree_patch[0].len() {
                visible_trees.extend(from_top(x, tree_patch));
                visible_trees.extend(from_bottom(x, tree_patch));
            }
            visible_trees.len()
        }

        fn from_left(y: usize, tree_patch: &TreePatch) -> Vec<Tree> {
            let line_size = tree_patch[0].len();
            let mut tallest_tree = tree_patch[y][0];
            let mut visible_trees: Vec<Tree> = vec![];
            visible_trees.push(Tree::new(0, y));
            for x in 0..line_size {
                if tree_patch[y][x] > tallest_tree {
                    tallest_tree = tree_patch[y][x];
                    visible_trees.push(Tree::new(x, y));
                }
            }
            visible_trees
        }

        fn from_right(y: usize, tree_patch: &TreePatch) -> Vec<Tree> {
            let line_size = tree_patch[0].len();
            let last_x = line_size - 1;
            let mut tallest_tree = tree_patch[y][last_x];
            let mut visible_trees: Vec<Tree> = vec![];
            visible_trees.push(Tree::new(last_x, y));
            for x in (0..line_size).rev() {
                if tree_patch[y][x] > tallest_tree {
                    tallest_tree = tree_patch[y][x];
                    visible_trees.push(Tree::new(x, y));
                }
            }
            visible_trees
        }

        fn from_top(x: usize, tree_patch: &TreePatch) -> Vec<Tree> {
            let mut tallest_tree = tree_patch[0][x];
            let mut visible_trees: Vec<Tree> = vec![];
            visible_trees.push(Tree::new(x, 0));
            for y in 0..tree_patch.len() {
                if tree_patch[y][x] > tallest_tree {
                    tallest_tree = tree_patch[y][x];
                    visible_trees.push(Tree::new(x, y));
                }
            }
            visible_trees
        }

        fn from_bottom(x: usize, tree_patch: &TreePatch) -> Vec<Tree> {
            let last_y = tree_patch.len() - 1;
            let mut tallest_tree = tree_patch[last_y][x];
            let mut visible_trees: Vec<Tree> = vec![];
            visible_trees.push(Tree::new(x, last_y));
            for y in (0..tree_patch.len()).rev() {
                if tree_patch[y][x] > tallest_tree {
                    tallest_tree = tree_patch[y][x];
                    visible_trees.push(Tree::new(x, y));
                }
            }
            visible_trees
        }
    }

    pub mod inside_out {
        use super::*;

        /// Computes scenic score for each tree in the TreePatch matrix and picks out the heighest
        pub fn get_highest_scenic_score(tree_patch: &TreePatch) -> usize {
            let mut highest_score = 0;
            for y in 0..tree_patch.len() {
                for x in 0..tree_patch[0].len() {
                    let score = get_scenic_score(x, y, tree_patch);
                    if score > highest_score {
                        highest_score = score;
                    }
                }
            }
            highest_score
        }

        fn get_scenic_score(x: usize, y: usize, tree_patch: &TreePatch) -> usize {
            to_left(x, y, tree_patch)
                * to_right(x, y, tree_patch)
                * to_top(x, y, tree_patch)
                * to_bottom(x, y, tree_patch)
        }

        fn to_left(x: usize, y: usize, tree_patch: &TreePatch) -> usize {
            let tree_in_consideration = tree_patch[y][x];
            let mut score = 0;
            for i in (0..x).rev() {
                score += 1;
                if tree_patch[y][i] >= tree_in_consideration {
                    return score;
                }
            }
            score
        }

        fn to_right(x: usize, y: usize, tree_patch: &TreePatch) -> usize {
            let tree_in_consideration = tree_patch[y][x];
            let mut score = 0;
            for i in (x + 1)..tree_patch[0].len() {
                score += 1;
                if tree_patch[y][i] >= tree_in_consideration {
                    return score;
                }
            }
            score
        }

        fn to_top(x: usize, y: usize, tree_patch: &TreePatch) -> usize {
            let tree_in_consideration = tree_patch[y][x];
            let mut score = 0;
            for i in (0..y).rev() {
                score += 1;
                if tree_patch[i][x] >= tree_in_consideration {
                    return score;
                }
            }
            score
        }

        fn to_bottom(x: usize, y: usize, tree_patch: &TreePatch) -> usize {
            let tree_in_consideration = tree_patch[y][x];
            let mut score = 0;
            for i in (y + 1)..tree_patch.len() {
                score += 1;
                if tree_patch[i][x] >= tree_in_consideration {
                    return score;
                }
            }
            score
        }
    }
}

/// Response to the first part
pub fn get_visible_trees(lines: impl Iterator<Item=String>) -> usize {
    tree_viewer::outside_in::get_visible_trees_count(&tree_patch_parser::parse_from_text(lines))
}

/// Response to the second part
pub fn get_highest_scenic_score(lines: impl Iterator<Item=String>) -> usize {
    tree_viewer::inside_out::get_highest_scenic_score(&tree_patch_parser::parse_from_text(lines))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_visible_trees() {
        assert_eq!(get_visible_trees("\
30373
25512
65332
33549
35390".lines().map(String::from)), 21);
    }

    #[test]
    fn test_get_highest_scenic_score() {
        assert_eq!(get_highest_scenic_score("\
30373
25512
65332
33549
35390".lines().map(String::from)), 8);
    }
}