use std::io;
use rand::Rng;

fn main() {
    print!("Hello Gentleman!\nWelcome to the guessing game!\n");
    print!("Want to play?\n(y/yes)\n");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line\n");

    choice = choice.trim().to_lowercase();

    if !(&choice == "y" || &choice == "yes") {
        print!("See you next time... Bye\n");
        return;
    }
    drop(choice);                                   //Freeing memory

    loop {
        let mut guesses_counter = 1;

        let rand_number = rand::thread_rng().gen_range(1, 11);  //range

        println!("Guess the number from 0 to 10!");
        loop {
            println!("Please input your guess.\n");     //printline

            let mut guess = String::new();   //Creating instance of string

            io::stdin()                             // Readline (scanf)
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: i32 = match guess.trim().parse() {        //String to int
                Ok(int_guess) => int_guess,
                Err(_) => -1

            };

            if guess == -1 {
                print!("\"{}\" is not a number\n", guess);
                continue;
            }

            if guess == rand_number{
                print!("You have won!!!!! The number was {}\n", guess);
                break;
            }

            if guesses_counter >5 {
                print!("Game over....\nThe secret number was {}\n", rand_number);
                break
            }

            println!("You guessed: {}", guess);

            guesses_counter += 1;
        }
        print!("Want to start again?\n(y/yes)\n");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line\n");

        choice = choice.trim().to_lowercase();

        if !(&choice == "y" || &choice == "yes") {
            print!("See you next time... Bye\n");
            return;
        }
        drop(choice);
    }
}
