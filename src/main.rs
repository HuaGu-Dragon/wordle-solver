use clap::{Parser, ValueEnum};
use wordle_solver::{Guesser, Wordle};

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser)]
#[command(name = "wordle_solver", version = "0.1.0", author = "HuaGu_Dragon")]
struct Cli {
    #[clap(value_enum, short, long, default_value_t = Implementation::Native)]
    implementation: Implementation,
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
        Implementation::Native => start(wordle_solver::algorithms::native::Native::new, cli.max),
        Implementation::Allocs => start(wordle_solver::algorithms::allocs::Allocs::new, cli.max),
        Implementation::Vexer => start(wordle_solver::algorithms::vexer::Vexer::new, cli.max),
    }
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
