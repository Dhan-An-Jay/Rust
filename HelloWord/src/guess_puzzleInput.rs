use std::io;
pub fn get_guess() {
    println!("Enter the Input-Guess :");
    let mut guess =  String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("Your Guess is {}",guess );
}