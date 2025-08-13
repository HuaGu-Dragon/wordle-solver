use std::collections::HashSet;

pub mod algorithms;

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

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guessers: G) -> Option<usize> {
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

    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

pub struct Guess {
    word: String,
    mask: [Correctness; 5],
}
impl Guess {
    fn matches(&self, word: &str) -> bool {
        assert_eq!(self.word.len(), 5);
        assert_eq!(word.len(), 5);
        let mut used = [false; 5];

        for (i, (a, g)) in word.chars().zip(self.word.chars()).enumerate() {
            if a == g {
                if self.mask[i] != Correctness::Correct {
                    return false;
                }
                used[i] = true;
            } else if self.mask[i] == Correctness::Correct {
                return false;
            }
        }

        for (g, e) in self.word.chars().zip(self.mask.iter()) {
            if *e == Correctness::Correct {
                continue;
            }
            if *e == Correctness::Misplaced
                && !word.chars().enumerate().any(|(i, a)| {
                    if a == g && !used[i] {
                        used[i] = true;
                        true
                    } else {
                        false
                    }
                })
            {
                return false;
            }
            if *e == Correctness::Wrong && word.chars().enumerate().any(|(i, a)| a == g && !used[i])
            {
                return false;
            }
        }

        true
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

impl<T> Guesser for T
where
    T: Fn(&[Guess]) -> String,
{
    fn guess(&mut self, history: &[Guess]) -> String {
        (self)(history)
    }
}

#[cfg(test)]
macro_rules! mask {
    (C) => {crate::Correctness::Correct};
    (M) => {crate::Correctness::Misplaced};
    (W) => {crate::Correctness::Wrong};
    ($($c:tt)+) => {[
        $(mask!($c)),+
    ]}
}

#[cfg(test)]
mod tests {

    mod guess_matcher {
        use crate::Guess;

        macro_rules! check {
            ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!(Guess {
                    word: $prev.to_string(),
                    mask: mask![$($mask )+]
                }
                .matches($next));
            };
            ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!Guess {
                    word: $prev.to_string(),
                    mask: mask![$($mask )+]
                }
                .matches($next));
            };
        }

        #[test]
        fn matches() {
            check!("apple" + [C C C C C] allows "apple");
            check!("apple" + [C C C C W] allows "appla");
            check!("apple" + [C C M W W] allows "apcdp");
            check!("baaaa" + [W C M W W] allows "aaccc");

            check!("apple" + [C C C C C] disallows "appla");
            check!("aaabb" + [C M W W W] disallows "accaa");
            check!("baaaa" + [W C M W W] disallows "caacc");
        }
    }

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
