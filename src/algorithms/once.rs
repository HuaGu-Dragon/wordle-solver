use std::{borrow::Cow, sync::OnceLock};

use crate::{Correctness, DICTIONARY, Guess, Guesser};

static INITIAL: OnceLock<Vec<(&'static str, usize)>> = OnceLock::new();

pub struct Once {
    remaining: Cow<'static, Vec<(&'static str, usize)>>,
}

impl Once {
    pub fn new() -> Self {
        Self {
            remaining: Cow::Borrowed(INITIAL.get_or_init(|| {
                Vec::from_iter(DICTIONARY.lines().map(|line| {
                    let (word, count) = line
                        .split_once(' ')
                        .expect("every line is word + space + frequency");
                    (word, count.parse().expect("frequency must be a number"))
                }))
            })),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
    word: &'static str,
    score: f64,
}

impl Guesser for Once {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            match self.remaining {
                Cow::Borrowed(remaining) => {
                    self.remaining = Cow::Owned(
                        remaining
                            .iter()
                            .filter(|&&(word, _)| last.matches(word))
                            .copied()
                            .collect(),
                    );
                }
                Cow::Owned(ref mut owned) => {
                    owned.retain(|&(word, _)| last.matches(word));
                }
            }
        }
        if history.is_empty() {
            return "tares".to_string();
        }

        let total: usize = self.remaining.iter().map(|&(_, count)| count).sum();

        let mut best: Option<Candidate> = None;
        for &(word, _) in self.remaining.iter() {
            let mut score = 0.0;

            for pattern in Correctness::patterns() {
                let mut pattern_count = 0;
                for (w, count) in self.remaining.iter() {
                    Guess {
                        word: Cow::Borrowed(w),
                        mask: pattern,
                    }
                    .matches(word)
                    .then(|| {
                        pattern_count += *count;
                    });
                }
                if pattern_count == 0 {
                    continue;
                }
                let p = pattern_count as f64 / total as f64;
                score -= p * p.log2();
            }

            if let Some(c) = best {
                if score > c.score {
                    best = Some(Candidate { word: word, score });
                }
            } else {
                best = Some(Candidate { word: word, score });
            }
        }
        best.expect("there should always be at least one candidate")
            .word
            .to_string()
    }
}
