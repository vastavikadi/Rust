// EXPLANATION FOR THE GUESSING GAME

//variables in rust are immutable by default, we use mut to make a variable mutable
let mut guess = String::new()//initializing th guess as string and with default of 0 value means empty string, mut so that in future we change the empty string to something we want

let apples = 5;//now this cannot be changed until we mut keyword


// io::stdin or std::io::stdin (if not imported in the top in the starting) - this handles the standard input

// the line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents)

// The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. you need to write &mut guess rather than &guess to make it mutable

//io::stdin().read_line(&mut guess).expect("Failed to read line"); - this is also correct but it is nice to break a long line in small parts while calling a method with syntax method()

// read_line() - puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant. Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and it contains the successfully generated value. The Err variant means the operation failed, and it contains information about how or why the operation failed.

// An instance of Result has an expect method that you can call. If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. 

// If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so that you can use it. In this case, that value is the number of bytes in the user’s input.


//expect crashes the program, it does not handle the error

// there is no crate in the standard lin of rust for random numbers and all, hence we will use rand crate - Remember that a crate is a collection of Rust source code files. The project we’ve been building  (guessing_game) is a binary crate, which is an executable. The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own.

// When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use. After updating the registry, Cargo checks the [dependencies] section and downloads any crates listed that aren’t already downloaded.  In this case, although we only listed rand as a dependency, Cargo also grabbed other crates that rand depends on to work


// When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.( lock file forces the cargo to only download the version specified in the lock leading to build the same way as the project owner did, with no dependency conflict - package-lock.json typa shit)

//cargo update ignores the lock file and downloads the latest version of the crates based on the toml file