use std::collections::HashMap;

use crate::{Correctness, DICTIONARY, Guess, Guesser};

pub struct Allocs {
    remaining: HashMap<&'static str, usize>,
}

impl Allocs {
    pub fn new() -> Self {
        Self {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(' ')
                    .expect("every line is word + space + frequency");
                (word, count.parse().expect("frequency must be a number"))
            })),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
    word: &'static str,
    score: f64,
}

impl Guesser for Allocs {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            self.remaining.retain(|word, _| last.matches(word));
        }
        if history.is_empty() {
            return "slate".to_string();
        }

        let total: usize = self.remaining.values().sum();

        let mut best: Option<Candidate> = None;
        for (&word, _) in self.remaining.iter() {
            let mut score = 0.0;

            for pattern in Correctness::patterns() {
                let mut pattern_count = 0;
                for (w, count) in self.remaining.iter() {
                    pattern_count += if Correctness::compute(w, word) == pattern {
                        *count
                    } else {
                        0
                    };
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
