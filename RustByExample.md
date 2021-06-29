# Rust By Example
## Comments
- `/// library docs for the following item`
- `//! library docs for the enclosing item`
- `cargo test --doc` runs document test cases
- `#[doc(inline)]` is used to inline docs
- `#[doc(no_inline)]` is used to prevent linking out to a separate page
- `[doc(hidden)]` using this tells `rustdoc` not to include this in documentation
- [library docs](https://doc.rust-lang.org/rust-by-example/meta/doc.html)

## Printing and Formatting
- `eprintln!` prints to `io::stderr`
- `fmt::Debug` uses `{:?}` marker
- `fmt:Display` uses `{}` marker
- `{<number>}` and `{<number>:?}` allow you to order inputs, with regular or debug
	- e.g. `println!("Item0 {0}, Item1 {1}, Item2: {2:?}", Zero, One, Two)`
- `{named:?}` can print named debug attributes
	- e.g. `println!("My name is {name:?}", name="Cory")`
- `{:#?}` pretty prints things
- `write!` writes a value, similar to `format!` with a formatter instance into a buffer
- `{:X}` formats a number as hexidecimal
- `{:o}` formats as an octal
- `{:.<number>}` lets you format based on a number of decimal points
	- e.g. `{:.2}`
- `{:0<number>}` is the amount of leading zeros
	- e.g. `{:06}`
- [Formatting Rules](https://doc.rust-lang.org/std/fmt/#formatting-traits)
- [More formatting resources](https://doc.rust-lang.org/std/fmt)

## Primitives
- You can prefix integers with `0x` for hexidecimal, `0o` for octal, or `0b` for binary.
- We can add suffixes to a number to indicate its type e.g. `10u32`, `1.5f64`.
- Arrays are declared with `[T, length]` and can be initalized the same way. 
	- e.g. `[0, 500]`

## Custom Types
- Aliases: `type ShorterName = SomeVeryLongNameOfAnEnumOrStruct;`
	- e.g. the `Self` alias

## Variable Bindings
- By setting a variable to itself you freeze the variablee until it goes out of scope.
	- e.g. `let mut num = 3; let num = num;`

## Types
- Casting to smaller types, e.g. `1000 as u8` functions like the modulus `1000 % 256`
- `#![allow(overflowing_literals)]` suppresses warnings from cast overflows
- `std::mem::size_of_val()` returns the byte size
- `#[allow(non_camel_case_types)]` silences warnings
- Use aliasing to reduce boilerplate

## Conversions
- `std::convert` crate
-  Use `From` and `Into` traits for generic conversions.
- `From` trait allows for a type to define how to create itself from another type.
- `Into` trait will usually require sepcification of the type to convert into since the compiler is unable to determine it most of the time.
	- e.g. `let num: f64 = 5u32.into();`
- `Into` is the reciprocal of the `From` trait. `.into()` calls `from()` for the type it is casting into.
- `TryFrom` and `TryInto` are the same as `From` and `Into`, except they are falliable conversions and return `Result<Self>`
	- Refer to https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html
	- This means you can call the `try_into()` similar to `into()` above.
- Implementing the `Display` trait gives you the `to_string()` method.
- turbofishing to a string works as long as the `FromStr` trait is implemented for that type. Though you don't need to use it, you can just declare the type with `:`
	- e.g. `"10".parse::<i32>().unwrap()`

## Expressions
- Single lines or `{}` are all expressions.
- last line in an expression without a semi-colon returns the evaluated expression
	- if there is a semi-colon then an empty tuple is returned `()`

## Control Flow
- Do not need to surround conditions with `()`
- loops can be named `'outer: loop {}`
- return a value out of a loop with the  `break`
	- e.g. `break counter * 2;`
- `for in` loops over an iterator by applying the 	`into_iter` function
	- `..` is a non-inclusive upper-range
	- `..=` is an incluse range
- `iter` borrows each element
- `into_iter` consumes the collection
- `iter_mut` borrows each element of the collection and allows it to be modified in place
- `match` statements can desctructure tuples, enums, pointers, and structs
	-  `match *reference` dereferences before the match
	-  `ref r =>` creates a reference on match
	-  `ref mut mr =>` creates a mutable reference on match
	-  matches can have guards with 
		-  e.g. `value if value == true =>`
	-  binding in match arms can be done with `@` and can be used with enums
		-  e.g. `n @ 1..=10 =>
		-  enum e.g. `Some(n @ 1..=10)`
-  `while let` is like `if let` but allows looping until the condition is met

## Functions