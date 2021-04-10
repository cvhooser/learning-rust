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

## Strategy to breakthrough
- RUST
  - I'm going to go through [The Rust Book](https://doc.rust-lang.org/book) in its entirety.
  - Then I plan to move on to [rustlings](https://github.com/rust-lang/rustlings) and see how challenging they are. 
  - After that I should have enough of an understanding to be able to start combing through open source projects code bases (polkadot) and make some bug fixes.
  - Then I want to attempt to write an implementation of the [RAFT](https://raft.github.io).
  - At this point I should have enough of an understanding to start applying for jobs.
- Distributed Systems
  - Starting from the beginning, read and note my way through these [Foundational Distributed Systems Papers](https://muratbuffalo.blogspot.com/2021/02/foundational-distributed-systems-papers.html?m=1).
  - Research and take notes on other consensus algorithms
- Blockchain
  - Research and understand the technicals of Polkadot.
  - Become an active member and contributor on the polkadot discord server.


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
- Finishing the [guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).
- error handling is done with the `Result` enum. Calling `.expect()` fails if the `Result::Err` enum has a value. Rust DOES NOT use `null`, but instead an enum `None`
- The `cargo doc --open` command, which will build documentation provided by all of your dependencies locally and open it in your browser. Amazing feature.
- Holy balls variable shadowing is amazing for type conversion!
- `:` is used for variable type annotation. An example being `let guess: us32 = ...`.
- The error handling in rust is so beautiful! Just look at this code 
```
let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
```
- 3.0
  - [Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
  - [Operators](https://doc.rust-lang.org/book/appendix-02-operators.html)

- 3.1
  - `const` must always be annotated with its type (use `const <var-name>: <type> = <value>;`) and is always immutable. i.e. cannot use `mut`.

- 3.2
  - Integers (i32 is default)
    - Signed (negative/positive): i8, i16, i32, i64, i128, isize
    - Unsigned (positive): u8, u16, u32, u64, u128, usize
    - isize and usize refer to the architecture of the machine the code runs on (32 vs 64 bit)
    - isize and usize are primarily used for indexing a collection
  - Integer Overflows
    - In debug mode the program will panic at runtime: `Unrecoverable Errors with panic!`
    - In release mode (using the `--release` flag), rust performs a **twos complement wrapping**.
      - Relying on this expected behavior is considered an error.
    - Solutions
      - Wrap in all modes with the `wrapping_*` methods
        - When wrapping is intended to happen e.g. modulus arithmetic
      - Return `None` if there is an overflow with the `checked_*` methods.
      - Return the value a boolean indicating whether there was overflow with the `overflowing_*` methods
      - Saturate at the values min or max values with `saturating_*` methods
        - Saturation stops values from being *greater than the max* or *less than the min*.
  - Floating Points (f64 is default)
    - f32 and f64
      - f64 is roughly the same speed as f32 but with more precision
      - f32 is single precision, f64 is double precision
  - Booleans
    - is either `true` or `false`.
    - uses `bool`, so `let f: bool = flase`
    - is one byte in size
  - Charaters
    - signified by `''`
    - is four bytes
    - Represents a Unicode Scalar Value
  - Compound Types
    - Tuples
      - tuples cannot grow or shrink once declared
      - uses `(, , ,)` syntax. e.g. `let tup: (i32, f64, u8) = (500, 6.4, 1);` or `let tup = (500, 6.4, 1);`
      - tuples can be destructured with pattern matching. I love destructuring with pattern matching!
      - tuples can be read by their index with `tup.[i]` e.g. `tup.0 == 500`
    - Arrays
      - all values must have the same type
      - arrays have a fixed length
      - uses `[, , , ]`
      - explicit syntax example `let a: [i32; 5] = [1, 2, 3, 4, 5];`
      - array is on the stack
      - If you don't know whether to use an array or a vector, use a vector
        - A vector is an array that can grow and shrink.
      - You can declare all the values a the start with `let a = [3; 5];`
        - This would be an array with all of length 5 all with a value of 3.
          - Can you use a generator in the first field?
      - access an element by `a[#]`
      - accessing an element outside the bounds of the array will cause a `panic!` i.e. runtime error.
        - The *panic* is due to a check at runtime that you are reading inside the bounds of the array. This means that you can not access invalid memory, unlike other languages.
        - e.g. *thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19 note: run with \`RUST_BACKTRACE=1\` environment variable to display a backtrace*


# Resources
- [Foundational Distributed Systems Papers](https://muratbuffalo.blogspot.com/2021/02/foundational-distributed-systems-papers.html?m=1)
- [99% Fault Tolerence](https://vitalik.ca/general/2018/08/07/99_fault_tolerant.html)
