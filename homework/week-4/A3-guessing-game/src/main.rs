use std::io;


fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess > secret {
        return 1;
    }
    if guess < secret {
        return -1;
    }
    return 0;
}


fn main() {
    let mut secret = 10;

    let mut guess = 0;
    let mut input = String::new();

    while guess != secret {

        input.clear();
        println!("Guess my secret number:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess: i32 = input.trim().parse().expect("Not a valid number");

    }
}
