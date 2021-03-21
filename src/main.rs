use rand::Rng;
// use instead of using, using random crate
fn main() {
    // looping using loop
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("Secret number {}", secret_number);
    println!("Guess the number game");
    println!("Enter your guess");
    loop {
        // mut makes the variable mutable
        let mut guess = String::new();
        // String::new() creates an empty instance of a string
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Converting string to unsigned 32bit int by trimming, prasing. can use same variable to parse it.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        /*
            std::io can be above the function to import the namespace then can use io::stdin
            It is like C++ std etc. string argument needs to be mutable so answers can change
            & is a reference to the variable without having to store new variables in memory to access it multiple times
            Refernces are immutable by default so need to add mut to make it mutable
            read_line returns a value in this case std::io::Result
        */
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too Low\n Enter a new Guess"),
            std::cmp::Ordering::Greater => println!("Too High\n Enter a new Guess"),
            std::cmp::Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
        println!("You guessed: {}", guess);
    }
}
