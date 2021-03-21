# Learning Rust

## Details

- Learning how to code in Rust by building a simple guessing game. 
- Rust commands and file structure:
    - Install rust
    - cargo new name_of_app
    - cargo build (to build app)
    - then use ./target/app_name to run it or
    - cargo run (build and run app)
    - all files in src folder

### Start

- Using cargo to build a new app
- Using the main.rs file to add code inside the main function
- Adding random number generator for user to guess
- Using namespaces e.g. the standard input output namespace for getting user input (std::io)

### Using rand crate

- Added rand to the dependencies in Cargo.toml to install and use the crate
- Using random generator from rand crate (use rand:Rng)
- Then using thread_rng function will generate a random number and then gen_range is the range

### Time to compare

- Using std::cmp::Ordering (Less, Greater or Equal Enum) to compare 2 values
- using match to check the values between 2 variables and reference to the secret_number
- parsing string to unsigned 32 bit int

### Time to loop

- Using loop keyword to loop
- break, breaks out of the loop
- Making program ignore non numbers (match the guess, if Ok(pass num) and return num, if Err(_) continue)