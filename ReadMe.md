# Starting Rust

# Resources
- [Go vs Rust](https://bitfieldconsulting.com/golang/rust-vs-go)
- [Go vs Rust idioms](https://programming-idioms.org/cheatsheet/Go/Rust)
- [Rust Pitfalls](https://docs.google.com/presentation/d/1-pvJCzwWKSlkiYkdC8FsFH5IRRX2a5UjT3_WhFB7hxE/edit#slide=id.gcbab3a369_1_258)
- [Install Rust](https://www.rust-lang.org/tools/install)
- [Getting Started](https://www.rust-lang.org/learn/get-started)
- [Cargo](https://doc.rust-lang.org/cargo/index.html)
- [Learn Rust](https://www.rust-lang.org/learn)
- [The Book](https://doc.rust-lang.org/book/)
- [Rustlings](https://github.com/rust-lang/rustlings/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
-[Rust Books](https://github.com/sger/RustBooks)

# Learning
## Day 1
- Tools
  - [VSCODE](https://code.visualstudio.com/) with [Rust Extension Pack](https://marketplace.visualstudio.com/items?itemName=swellaby.rust-pack)

- [Getting Started](https://www.rust-lang.org/learn/get-started)

- Build Tools
  - Cargo is a build tool that helps manage dependencies known as `crates`
  - Project information and dependencies exist in `Cargo.toml`

- Compiling
  - `rustc` is used to compile the code
  - `cargo build` installs dependencies
    - compiles into `./target/debug/<project-name>`
    - creates a `cargo.lock` file for dependency tracking
  - `cargo run` compiles and runs the program
  - `cargo check` will tell us if you program compiles or not
  - `cargo build --release` will build with optimizations for releases

- Formatting
  - For single files you can use `rustfmt <file>.rs`
      - The default is 4 spaces (I hate that) so I adjusted it to 2 for sanity sake, besides that I left everything the same so I can learn idomatic Rust
  - For `cargo` projects you can run `cargo fmt` to format the entire project
  - You can add a `rustfmt.toml` file to the project directory that will be used with both format commands.
  - Add a `--check` flag to see the changes (if any) that would be applied when you change

- Syntax
  - `<command>!` refers to a macro, NOT a function call

## Day 2
- [Debugger](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
  - launch.json
  ```
      {
      "version": "0.2.0",
      "configurations": [
        {
          "type": "lldb",
          "request": "launch",
          "name": "Debug",
          "program": "${workspaceFolder}/<directory>/target/debug/<program-entry>",
          "args": [],
          "cwd": "${workspaceFolder}/<directory>/target/debug/",
          "sourceLanguages": ["rust"]
        }
      ]
    }

  ```
- `unwap` is an error handler that uses `Result<T, E>` enum to return the value or the `panic!` the error.

## Day 3
- Certain packages are auto included.Theses are referred to as the [prelude](https://doc.rust-lang.org/std/prelude/index.html).
- [Guessing Game Tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
  - Finished up to: _**"Generating a Random Number"**_
- Variables are immutable by default, but `mut` will allow a variable to be mutable.
- `::` calls a function from a namespace.
- `use` keyword allows us to remove the leading namespace of the imported crate
- `&` is to get something by reference (or the memory address of where the variable is)
- I created an offline copy of the [rust book](https://github.com/rust-lang/book).

## Day 4
- Reading
  - [Raft Consensus Paper](https://raft.github.io/raft.pdf)

## Day 5
- Reading
  - [Time, Clocks, and the Ordering of Events in a Distributed System]( https://lamport.azurewebsites.net/pubs/time-clocks.pdf)

## Day 6
- Reading
  - [Thoughts on Rust Bloat](https://raphlinus.github.io/rust/2019/08/21/rust-bloat.html)

## Day 7

# Resources
- [Foundational Distributed Systems Papers](https://muratbuffalo.blogspot.com/2021/02/foundational-distributed-systems-papers.html?m=1)
- [99% Fault Tolerence](https://vitalik.ca/general/2018/08/07/99_fault_tolerant.html)
