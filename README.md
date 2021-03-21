# Learning Rust

## Details

- Learning how to code in Rust by building a simple guessing game. 

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
- parsing string to 32 bit int