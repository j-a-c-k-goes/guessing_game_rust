// use Range from random crate
use rand::Rng;
// cmp::Ordering from std
use std::cmp::Ordering; 
// import the i/o library from the standard library (std), basically: "use i/o from standard" // Rng from random
use std::io; 

fn main() {
    // invoke game title
    game_title();
    // bind secret_number to a randomly threaded value in specified range
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // invoke game prompt
    game_prompt();

    // begin loop
    loop {
        // bind guess to an input, which should be a string (inferred)
        // this will be later shadowed b/c:
        // secret_number is a number type, this is a string type, they do not match
        let mut guess = String::new();

        // using i/o
        // read the line, 
        // if not a guess, return error message
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

        // shadow the previous guess value
        // used for conerting a value to another type
        // bind guess to a match expression
        // ignore a non number guess
        // error handling > crashing on error
        // .trim() removes whitespace
        // .parse() anlyzes string into a a number, returning  a result type
        // ':' after guess means variable's type is being annotated
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // use catchall to consider all error values
            Err(_) => continue,
        };

        // print the guessed string
        println!("you guessed:\t{}", guess);

        // match statement comparing guess to secret number
        match guess.cmp(&secret_number) {
            // 
            Ordering::Less => println!("too small\ttype 'quit' to stop playing"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("winner");
                println!("the secret number\t{}", secret_number);
                // break loop upon correct guess
                break;
            }
        }
    }
}
// .......... give the game a title
fn game_title() {
    let title = "\nGUESS THE NUMBER\n";
    println!("{}", title);
}
// .......... give the game a prompt
fn game_prompt() {
    println!("enter a guess.");
}
