use rand::Rng;
// use instead of using, using random crate
fn main() {
    println!("Guess the number game");
    println!("Enter your guess");
    // mut makes the variable mutable
    let mut guess = String::new();
    // String::new() creates an empty instance of a string
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number {}", secret_number);
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    /*
       std::io can be above the function to import the namespace then can use io::stdin
       It is like C++ std etc. string argument needs to be mutable so answers can change
       & is a reference to the variable without having to store new variables in memory to access it multiple times
       Refernces are immutable by default so need to add mut to make it mutable
       read_line returns a value in this case std::io::Result
    */
    println!("You guessed: {}", guess);
}
