use wordle_solver::Wordle;

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let wordle = Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = wordle_solver::algorithms::native::Native::new();
        if let Some(time) = wordle.play(answer, guesser) {
            println!("Solved {answer} in {time} guesses");
        } else {
            println!("Failed to solve {answer}");
        }
    }
}
