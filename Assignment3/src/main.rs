// Function to check the guess against the secret number
fn check_guess(guess:i32, secret:i32) -> i32
{
    if guess==secret {
       0
    }
    else if guess > secret {
        1
    }
    else{
        -1
    }
}


fn main() {
    let secret = 32;
    let mut num_guesses = 0; 
    let mut guess = 22; 

    loop {
        num_guesses += 1; 

        match check_guess(guess, secret) {
            0 => {
                println!("Your guess is correct!");
                break; 
            }
            1 => println!("Your guess is too high."),
            -1 => println!("Your guess is too low."),
            _ => unreachable!(),
        }

        guess += 1;
    }

    println!("It took you {} guesses", num_guesses);
}
