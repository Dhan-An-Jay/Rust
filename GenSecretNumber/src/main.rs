
use rand::Rng;
use std::io;
use std::cmp::Ordering;

#[allow(non_snake_case)]

/*fn main() {
    println!("Enter Your Guess");
    let mut guess:u8 =7; 
    //io::stdin().read(guess: &mut [u8]).expect("not matced ");
    print!("Your Guess is {}\n",guess );
    let secret_number= rand::thread_rng().gen_range(1,101);
    print!("Your Secret Number is {}\n",secret_number);
    
    match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
    }

}*/
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You gave not entered number so it will make problem in below comparison: {}", guess);
                continue
                },
        };

        println!("You guessed: {} is compared with Secret Number {}", guess, secret_number);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Result of Comparision is >>>> Too small!"),
            Ordering::Greater => println!("Result of Comparision is >>>> Too big!"),
            Ordering::Equal => println!("Result of Comparision is >>>> You win!"),
        }
    }
}
