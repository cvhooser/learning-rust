## Day 3
- Certain packages are auto included.Theses are referred to as the [prelude](https://doc.rust-lang.org/std/prelude/index.html).
- [Guessing Game Tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
	- Finished up to: _**"Generating a Random Number"**_
- Variables are immutable by default, but `mut` will allow a variable to be mutable.
- `::` calls a function from a namespace.
- `use` keyword allows us to remove the leading namespace of the imported crate
- `&` is to get something by reference (or the memory address of where the variable is)
- I created an offline copy of the [rust book](https://github.com/rust-lang/book).
- Finishing the [guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).
- error handling is done with the `Result` enum. Calling `.expect()` fails if the `Result::Err` enum has a value. Rust DOES NOT use `null`, but instead an enum `None`
- The `cargo doc --open` command, which will build documentation provided by all of your dependencies locally and open it in your browser. Amazing feature.
- Holy balls variable shadowing is amazing for type conversion!
- `:` is used for variable type annotation. An example being `let guess: us32 = ...`.
- The error handling in rust is so beautiful! Just look at this code 

  ```rust
  let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };
  ```
