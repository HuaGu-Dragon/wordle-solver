use clap::{Parser, ValueEnum};
use std::io::Write;
use wordle_solver::{Correctness, Guess, Guesser, Wordle};

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser)]
#[command(name = "wordle_solver", version = "0.1.0", author = "HuaGu_Dragon")]
struct Cli {
    #[clap(value_enum, short, long, default_value_t = Implementation::Native)]
    implementation: Implementation,

    #[clap(short, long, default_value_t = false)]
    play: bool,

    #[arg(short, long)]
    max: Option<usize>,
}

#[derive(ValueEnum, Clone, Copy)]
enum Implementation {
    Native,
    Allocs,
    Vexer,
}

fn main() {
    let cli = Cli::parse();

    match cli.implementation {
        Implementation::Native => {
            if cli.play {
                guess(wordle_solver::algorithms::native::Native::new)
            } else {
                start(wordle_solver::algorithms::native::Native::new, cli.max)
            }
        }
        Implementation::Allocs => {
            if cli.play {
                guess(wordle_solver::algorithms::allocs::Allocs::new)
            } else {
                start(wordle_solver::algorithms::allocs::Allocs::new, cli.max)
            }
        }
        Implementation::Vexer => {
            if cli.play {
                guess(wordle_solver::algorithms::vexer::Vexer::new)
            } else {
                start(wordle_solver::algorithms::vexer::Vexer::new, cli.max)
            }
        }
    };
}

fn start<G: Guesser>(mut mk: impl FnMut() -> G, max: Option<usize>) {
    let wordle = Wordle::new();
    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = mk();
        if let Some(time) = wordle.play(answer, guesser) {
            println!("Solved {answer} in {time} guesses");
        } else {
            println!("Failed to solve {answer}");
        }
    }
}

fn guess<G: Guesser>(mut mk: impl FnMut() -> G) {
    let mut history = Vec::new();
    let mut guesser = mk();
    for _ in 0..6 {
        let guess = guesser.guess(&history);

        let mut stdout = std::io::stdout();
        writeln!(
        stdout,
        "Guess: {guess}\nPlease enter the correctness pattern (C for Correct, M for Misplaced, W for Wrong):"
    ).expect("Failed to write to stdout");

        let stdin = std::io::stdin();
        let mut pattern = String::new();
        stdin.read_line(&mut pattern).expect("Failed to read line");
        let mask = pattern
            .trim()
            .bytes()
            .filter(|v| !v.is_ascii_whitespace())
            .map(|c| match c {
                b'C' => Correctness::Correct,
                b'M' => Correctness::Misplaced,
                b'W' => Correctness::Wrong,
                c => panic!("Invalid character in pattern {c}"),
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Pattern must be 5 characters long");

        history.push(Guess {
            word: std::borrow::Cow::Owned(guess),
            mask,
        });
    }
}
