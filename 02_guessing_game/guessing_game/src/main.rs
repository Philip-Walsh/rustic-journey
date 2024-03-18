use rand::Rng;
use std::cmp::Ordering;
use std::io;

const banner: &str = "\n _______ __   __ _______ _______ _______          \n\
|       |  | |  |       |       |       |         \n\
|    ___|  | |  |    ___|  _____|  _____|         \n\
|   | __|  |_|  |   |___| |_____| |_____          \n\
|   ||  |       |    ___|_____  |_____  |         \n\
|   |_| |       |   |___ _____| |_____| |         \n\
|_______|_______|_______|_______|_______|         \n\
\t  _______ __   __ _______              \n\
\t |       |  | |  |       |             \n\
\t |_     _|  |_|  |    ___|             \n\
\t   |   | |       |   |___              \n\
\t   |   | |       |    ___|             \n\
\t   |   | |   _   |   |___              \n\
\t   |___| |__| |__|_______|             \n\
__    _ __   __ __   __ _______ _______ ______   \n\
|  |  | |  | |  |  |_|  |  _    |       |    _ |  \n\
|   |_| |  | |  |       | |_|   |    ___|   | ||  \n\
|       |  |_|  |       |       |   |___|   |_||_ \n\
|  _    |       |       |  _   ||    ___|    __  |\n\
| | |   |       | ||_|| | |_|   |   |___|   |  | |\n\
|_|  |__|_______|_|   |_|_______|_______|___|  |_|";


fn main() {
    println!("{}", banner);

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter your guess");


        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!\n");
                break;
            },
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n")
        }
    }
}