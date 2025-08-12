use std::collections::HashSet;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(
                DICTIONARY
                    .lines()
                    .map(|line| line.split_once(' ').unwrap().0),
            ),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, guessers: G) -> Option<usize> {
        let mut history = Vec::new();
        for i in 1..=32 {
            let guess = guessers.guess(&history);
            if guess == answer {
                return Some(i);
            }
            assert!(self.dictionary.contains(guess.as_str()));
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: guess,
                mask: correctness,
            });
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

impl Correctness {
    pub fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Wrong; 5];
        let mut used = [false; 5];

        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
                used[i] = true;
            }
        }

        for (i, g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
                continue;
            }
            if answer.chars().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    true
                } else {
                    false
                }
            }) {
                c[i] = Correctness::Misplaced;
            }
        }

        c
    }
}

pub struct Guess {
    word: String,
    mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&self, history: &[Guess]) -> String;
}

impl<T> Guesser for T
where
    T: Fn(&[Guess]) -> String,
{
    fn guess(&self, history: &[Guess]) -> String {
        (self)(history)
    }
}

#[cfg(test)]
mod tests {

    mod game {
        use crate::{Guess, Wordle};

        #[test]
        fn genius() {
            let w = Wordle::new();
            assert_eq!(w.play("apple", |_: &[Guess]| "apple".to_string()), Some(1));
        }

        #[test]
        fn magnificent() {
            let w = Wordle::new();
            assert_eq!(
                w.play("apple", |guess: &[Guess]| if guess.len() == 1 {
                    "apple".to_string()
                } else {
                    "arise".to_string()
                }),
                Some(2)
            );
        }

        #[test]
        fn impressive() {
            let w = Wordle::new();
            assert_eq!(
                w.play("apple", |guess: &[Guess]| if guess.len() == 2 {
                    "apple".to_string()
                } else {
                    "arise".to_string()
                }),
                Some(3)
            );
        }

        #[test]
        fn splendid() {
            let w = Wordle::new();
            assert_eq!(
                w.play("apple", |guess: &[Guess]| if guess.len() == 3 {
                    "apple".to_string()
                } else {
                    "arise".to_string()
                }),
                Some(4)
            );
        }

        #[test]
        fn great() {
            let w = Wordle::new();
            assert_eq!(
                w.play("apple", |guess: &[Guess]| if guess.len() == 4 {
                    "apple".to_string()
                } else {
                    "arise".to_string()
                }),
                Some(5)
            );
        }

        #[test]
        fn phew() {
            let w = Wordle::new();
            assert_eq!(
                w.play("apple", |guess: &[Guess]| if guess.len() == 5 {
                    "apple".to_string()
                } else {
                    "arise".to_string()
                }),
                Some(6)
            );
        }

        #[test]
        fn oops() {
            let w = Wordle::new();
            assert_eq!(
                w.play("apple", |_guess: &[Guess]| "arise".to_string()),
                None
            );
        }
    }

    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("apple", "apple"), mask![C C C C C]);
        }

        #[test]
        fn all_gray() {
            assert_eq!(Correctness::compute("abcde", "fghij"), mask![W W W W W]);
        }

        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute("abcde", "eabcd"), mask![M M M M M]);
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask![C C W W W]);
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute("aabbb", "ccaac"), mask![W W M M W]);
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute("aabbb", "caacc"), mask![W C M W W]);
        }

        #[test]
        fn dremann_from_chat() {
            assert_eq!(Correctness::compute("azzaz", "aaabb"), mask![C M W W W]);
        }

        #[test]
        fn itsapoque_from_chat() {
            assert_eq!(Correctness::compute("baccc", "aaddd"), mask![W C W W W]);
        }

        #[test]
        fn ricoello_from_chat() {
            assert_eq!(Correctness::compute("abcde", "aacde"), mask![C W C C C]);
        }
    }
}
