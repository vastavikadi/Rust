// Cargo is rust's build system and package manager and it helps you manage your Rust projects. It allows you to easily build, run, and test your code, as well as manage dependencies and publish your packages to crates.io. With Cargo, you can create new projects, add external libraries, and keep track of your project's metadata. It simplifies the process of building and distributing Rust applications, making it an essential tool for Rust developers.

//cargo --version // checks the version of Cargo installed on your system.

//cargo new my_project // creates a new Rust project named "my_project" with a default directory structure and a Cargo.toml file that contains metadata about the project, such as its name, version, and dependencies. The command also creates a src directory with a main.rs file that contains a simple "Hello, world!" program. 

// Cargo.toml (Tom’s Obvious, Minimal Language)
// [package]
// name = "hello_cargo"
// version = "0.1.0"
// edition = "2024"//edition of rust to be used for the project

// [dependencies]

//In rust, packages are called crates. A crate is a compilation unit in Rust, and it can be either a binary crate (an executable) or a library crate (a reusable library). Crates can depend on other crates, and Cargo makes it easy to manage these dependencies. When you specify dependencies in the Cargo.toml file, Cargo will automatically download and compile them for you, ensuring that your project has access to the necessary libraries.

// Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

//cargo init // initializes a new Cargo project in the current directory. It creates a Cargo.toml file and a src directory with a main.rs file if they do not already exist. This command is useful for setting up a new Rust project in an existing directory.

//cargo build // compiles the Rust code in the current project and generates an executable file in the target/debug directory. The build command also checks for any errors or warnings in the code and displays them in the console. If the build is successful, you can run the executable using the cargo run command.

//cargo run // compiles the Rust code in the current project (if necessary) and runs the resulting executable. This command is a convenient way to build and run your code in one step. If there are any errors or warnings in the code, they will be displayed in the console before the program is executed.


//cargo check // checks the Rust code in the current project for errors and warnings without generating an executable file. This command is useful for quickly verifying that your code is syntactically correct and free of common mistakes. It can help you catch issues early in the development process before you attempt to build or run your code.

//cargo test // runs the tests defined in the current project. Rust has a built-in testing framework that allows you to write unit tests and integration tests for your code. The cargo test command compiles the code and executes the tests, reporting any failures or errors in the console. This command is essential for ensuring that your code behaves as expected and meets the desired requirements.

//cargo doc // generates documentation for the current project and its dependencies. Rust has a built-in documentation system that allows you to write comments in your code that can be transformed into HTML documentation. The cargo doc command compiles the code and generates the documentation, which can be viewed in a web browser. This command is useful for creating reference materials for your code and sharing it with others.

//cargo build ---release // compiles the Rust code in the current project with optimizations enabled, generating an executable file in the target/release directory. This command is used when you want to create a production-ready version of your code that runs faster and uses fewer resources. The release build takes longer to compile than the debug build, but it produces a more efficient executable.

// running the cargo doc --open command will build documentation provided by all your dependencies locally and open it in your browser. If you’re interested in other functionality in the rand crate, for example, run cargo doc --open and click rand in the sidebar on the left.