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
    let mut guess_count = 0;

    while guess != secret {

        input.clear();
        println!("Guess my secret number:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        guess = input.trim().parse().expect("Not a valid number");
        
        let result = check_guess(guess, secret);
        if result == 1 {
            println!("Too High!");
        }
        else if result == -1 {
            println!("Too Low!");
        }
        else {
            println!("You Got It!");
        }
        guess_count += 1;
    }
    println!("Total Guesses: {}", guess_count);

}
