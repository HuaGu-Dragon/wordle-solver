use std::{borrow::Cow, collections::BTreeMap, sync::OnceLock};

use crate::{Correctness, DICTIONARY, Guess, Guesser};

static INITIAL: OnceLock<Vec<(&'static str, usize)>> = OnceLock::new();

static MATCHES: OnceLock<BTreeMap<(&'static str, &'static str, [Correctness; 5]), bool>> =
    OnceLock::new();

pub struct Precalc {
    remaining: Cow<'static, Vec<(&'static str, usize)>>,
}

impl Precalc {
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

impl Guesser for Precalc {
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
                    let cache = MATCHES.get_or_init(|| {
                        let mut out = BTreeMap::new();

                        for (word1, _) in self.remaining.iter() {
                            for (word2, _) in self.remaining.iter() {
                                if word2 < word1 {
                                    continue;
                                }
                                for pattern in Correctness::patterns() {
                                    let an_guess = Guess {
                                        word: Cow::Borrowed(word1),
                                        mask: pattern,
                                    }
                                    .matches(word2);
                                    out.insert((*word1, *word2, pattern), an_guess);
                                }
                            }
                        }

                        out
                    });

                    let key = if word < w {
                        (word, *w, pattern)
                    } else {
                        (*w, word, pattern)
                    };

                    if cache.get(&key).copied().unwrap_or_else(|| {
                        Guess {
                            word: Cow::Borrowed(w),
                            mask: pattern,
                        }
                        .matches(word)
                    }) {
                        pattern_count += count;
                    }
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
