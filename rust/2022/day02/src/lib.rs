pub mod input;

enum PlayOutcome{
    WIN = 6,
    DRAW = 3,
    LOSE = 0,
}

#[derive(PartialEq, Copy, Clone)]
enum RPS { ROCK, PAPER, SCISORS }

impl RPS {
    fn new(shape: char) -> Self {
        match shape {
            'A' => RPS::ROCK,
            'B' => RPS::PAPER,
            'C' => RPS::SCISORS,
            _ => panic!("Invalid opponent shape character. Only allowed A, B or C")
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            RPS::ROCK => RPS::SCISORS,
            RPS::PAPER => RPS::ROCK,
            RPS::SCISORS => RPS::PAPER,
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            RPS::ROCK => RPS::PAPER,
            RPS::PAPER => RPS::SCISORS,
            RPS::SCISORS => RPS::ROCK,
        }
    }

    fn round_score(&self, opponent_shape: RPS) -> usize {
        self.play(opponent_shape) + self.score()
    }

    fn play(&self, opponent_shape: RPS) -> usize {
        (if self.wins_against() == opponent_shape {
            PlayOutcome::WIN
        } else if self.loses_against() == opponent_shape {
            PlayOutcome::LOSE
        } else {
            PlayOutcome::DRAW
        }) as usize
    }

    fn score(&self) -> usize {
        match self {
            RPS::ROCK => 1,
            RPS::PAPER => 2,
            RPS::SCISORS => 3,
        }
    }
}

/*
    We create an enum to hold these shapes so as to help with validation. This way, if I have a
    MyShape or OpponentShape, I know characters are bound to be one of these matches and I don't
    have to check everywhere else in the program.
 */
enum MyShape {X, Y, Z}

impl MyShape {
    fn new(shape: char) -> Self {
        match shape {
            'X' => MyShape::X,
            'Y' => MyShape::Y,
            'Z' => MyShape::Z,
            _ => panic!("Invalid my shape character. Only allowed X, Y or Z")
        }
    }
}

impl From<MyShape> for RPS {
    fn from(my_shape: MyShape) -> Self {
        match my_shape {
            MyShape::X => RPS::ROCK,
            MyShape::Y => RPS::PAPER,
            MyShape::Z => RPS::SCISORS,
        }
    }
}

enum MyOutcome {
    X(RPS),
    Y(RPS),
    Z(RPS)
}

impl MyOutcome {
    fn new(shape: char, opponent_shape: RPS) -> Self {
        match shape {
            'X' => MyOutcome::X(opponent_shape),
            'Y' => MyOutcome::Y(opponent_shape),
            'Z' => MyOutcome::Z(opponent_shape),
            _ => panic!("Invalid my outcome character. Only allowed X, Y or Z")
        }
    }
}

impl From<MyOutcome> for RPS {
    fn from(my_outcome: MyOutcome) -> Self {
        match my_outcome {
            MyOutcome::X(opponent_shape) => opponent_shape.wins_against(),
            MyOutcome::Y(opponent_shape) => opponent_shape,
            MyOutcome::Z(opponent_shape) => opponent_shape.loses_against(),
        }
    }
}

pub fn total_rps_score_with_my_shape(plays: impl Iterator<Item=impl AsRef<str>>) -> usize {
    let mut total_score = 0;
    for line in plays {
        if let Some((opponent_shape, my_shape)) = line.as_ref().trim().split_once(' ') {
            let opponent_shape = RPS::new(opponent_shape.chars().next().expect("Can at most be empty"));
            let my_shape = MyShape::new(my_shape.chars().next().expect("Can at most be empty"));
            total_score += RPS::from(my_shape).round_score(opponent_shape);
        }
    }
    total_score
}

pub fn total_rps_score_with_outcome(plays: impl Iterator<Item=impl AsRef<str>>) -> usize {
    let mut total_score = 0;
    for line in plays {
        if let Some((opponent_shape, my_shape)) = line.as_ref().trim().split_once(' ') {
            let opponent_shape = RPS::new(opponent_shape.chars().next().expect("Can at most be empty"));
            let my_outcome = MyOutcome::new(my_shape.chars().next().expect("Can at most be empty"), opponent_shape);
            total_score += RPS::from(my_outcome).round_score(opponent_shape);
        }
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_rpx_score_with_my_play_one_play() {
        assert_eq!(total_rps_score_with_my_shape("\
A Y".lines()), 8)
    }

    #[test]
    fn test_total_rpx_score_with_my_play_three_plays() {
        assert_eq!(total_rps_score_with_my_shape("\
A Y
B X
C Z".lines()), 15)
    }

    #[test]
    fn test_total_rpx_score_with_outcome_one_play() {
        assert_eq!(total_rps_score_with_outcome("\
A Y".lines()), 4)
    }

    #[test]
    fn test_total_rpx_score_with_outcome_three_plays() {
        assert_eq!(total_rps_score_with_outcome("\
A Y
B X
C Z".lines()), 12)
    }
}