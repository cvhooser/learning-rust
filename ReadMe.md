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

### 3 Common Programming Concepts
- [Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
- [Operators](https://doc.rust-lang.org/book/appendix-02-operators.html)

#### 3.1 Variables and Mutability
- `const` must always be annotated with its type (use `const <var-name>: <type> = <value>;`) and is always immutable. i.e. cannot use `mut`.

#### 3.2 Data Types
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
  - *Note* You can use `_` to separate longer integers for readibility. e.g. `1_000_000` = `1000000`
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

## Day 8
#### 3.3 Functions
- declared with `fn`
  - `fn <name>() { }`
- Rust uses *snake_case* for functions and variable names
- order of function declartion does not matter
- Function signatures must declare the type of each parameter.
- Rust is an expression based language and functions are made up of statements and expressions
  - *Statements* are instructions that perform some action and do not return a value
  - *Expressions* evaluate to a resulting value
    - Calling a function is an expression
    - Calling a macro is an expression
    - A block used to create a scope i.e. `{ }`, is an expression
    - **expressions do not end with a semicolon, *if an expression ends with a semicolon it becomes a statement that will not return a value***
      - e.g.
      ```
      fn main() {
        let x = 5;

        let y = {
          let x = 3;
          x + 1 //Note the missing semicolon here
        };
        println!("The value of y is: {}", y);
      }
      ```
  - `let y = 6` does not return a value
    - *note* other languages like ruby and c return the value of their assignment, which allows `x = y = 6`
- Return values in function declarations use `->` e.g. `fn plus_one(x: i32) -> i32 { x + 1 }`
  - You can use the `return` keyword explicitly and use a `;` with an expression and it will not become a statement.
    - e.g. `fn plus_one(x: i32) -> i32 { return x + 1; }`
      
#### 3.4 Comments
- `//` thats it for now

#### 3.5 Control Flow
- Conditionals
  - `if` conditions do not need surrounding `()`
    - Blocks of code inside a condition `{}` is sometimes called *arms*, like in pattern matching.
  - Implicit conversions do not happen for conditionals
    - e.g. `let number = 3; if number { }` will throw an error because it got an integer instead of a boolean. We instead need to use `let number = 3; if number != 0 { }`
  - You can use multiple conditions with `else if`
  - You can use conditionals in let statements
    - `let number = if <condition> { 5 } else { 6 };` *note: all expression results must be the same type*
- Loops
  - three kinds of loops: `loop`, `while`, and `for`
  - for uses iteraters `for <value> in <collection>.iter`
    - e.g. `let a = [1,2,3]; for element in a.iter() { }`

### 4.0 Understanding Ownership
- allows Rust to make memory safety guarantees without needing a garbage collector. Features include borrowing and splices. 

#### 4.1 What is Ownership?
- Memory is managed by a system of ownership that the compiler checks at compile time (does not effect runtime)
- Data on the stack must have a known, fixed size. Dyanmically sized data sits on the heap.
- Ownership exists as a way to manage heap data.
- Rules
  - Each value in Rust has a variable that's called its owner.
  - There can only be one owner at a time
  - When the owner goes out of scope, the value will be dropped.
- Variable Scope
  - Basically after it is declared, inside of `{}`
- Memory and Allocation
  - Memory is allocated when initializing a variable (allocates memory on the heap), it is freed when the variable goes out of scope (rust automatically calls a `drop` function).
    - This is similar to a C++ pattern called *Resource Acquisition Is Initalization (RAII)*
- Ways Variables and Data Interact:Move
  - Reassignment copies over stack values to the new variable, NOT heap values.
    - e.g. `let s1 = String::from("hello"); let s2 = s1;`
    - When `s2` and `s1` go out of scope a *double free* will occur. Where drop tries to free both `s1` and `s2` above.
    - To avoid this rust considers `s1` to no longer be valid and does not need to free anything when it goes out of scope.
    - This also means that s1 cannot be used after s2 is declared either.
    - Easier put: "Reallocation creates a dead variable."
    - Instead of being called a *shallow copy* this is called a *move* (due to the invalidation of the previous variable)
    - Rust will NEVER automatically create "deep" copies of your data. Thus all *automatic* copying is inexpensive at runtime. Deep copying can be done with `.clone()`
- Ownership and Functions
  - Variables go out of scope if they are on the heap, and passed into a funtion (see ownership example code). They are *moved* into the function.
  - The same applies for variables that are returned from a function. They are *moved* to the location the function is returning to.

## Day 9
#### 4.2 References and Borrowing
- `&` lets you create a reference that does not own a variable.
- Having references as function parameters is known as borrowing.
- You CANNOT change something you are borrowing.
- Mutable References
  - You can change it if you make it a mutable reference
  - You can only have ONE mutable reference to a particular piece of data in a particular scope.
    - This prevents a data race from occuring
      * Two or more pointes access The same data at the same time.
      * At least one of the pointers is being used to write to the data.
      * There's no mechanism being used to synchronize access to the data.
    - You cannot mix mutable and immutable references
      - e.g. 
      ```
          let mut s = String::from("hello");
          let r1 = &s; // no problem
          let r2 = &s; // no problem
          let r3 = &mut s; // BIG PROBLEM
          println!("{}, {}, and {}", r1, r2, r3);
      ```
    - *Note* A references scope starts where it is introduced and ends on its last usage.
      - That is why this example works:
      ```
          let mut s = String::from("hello");
          let r1 = &s; // no problem
          let r2 = &s; // no problem
          println!("{} and {}", r1, r2);
          // r1 and r2 are no longer used after this point
          let r3 = &mut s; // no problem
          println!("{}", r3);
      ```
- Dangling References
  - Having reference to a memory for a variable that had previously gone out of scope.
    - Of course Rust protects against this
- Overview
  - At any given time you can have either one mutable reference or multiple immutable references
  - References will always be valid

#### 4.3 The Slice Type
- Lets you reference a contiguous sequence of elements in a collection.
- String Slices
  ```
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
  ```
  - Leading and trailing numbers aren't necessary for the beginning and the end.
    e.g. `&s[..2]` and `&s[3..]`
  - *Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.*
  - the function signature for string slices is `fn first_word(s: &str) -> &str { }`
  - String literals are also string slices
- Other Slices
  - There is a general array slice with the same syntax as a string slice.

## Day 10 / 11
### 5.0 Using Structs to Structure Related Data
- custom data type that lets you name and package together multiple related values that make up a meaningful group.

#### 5.1 Defining and Instantiating Structs
- Defined with the `struct` keyword
  - e.g. `struct User { name: String, email: String}`
- Instantiated the same was as defined but with values.
  - e.g. `let user1 = User {name: "Cory", email: "cory@mail.com"}`
- Values can be called with dot notation
  - e.g. `user1.email`
- If the instance is mutable we can use dot notation to change the value.
  - *note*: An entire struct must be mutable. You CANNOT make just some fields mutable.
- You can use the *field init shorthand syntax* to instantiate structs without having to restate the field name if it is the same as the variable name (like javascript)
  - e.g.
  ```
  let email = email@somewhere.com;
  let username = johnsandman;
  let active = true;
  let sign_in_count = 1;
  let user1 = new User {email, name, active, sign_in_count}
  ```
- *struct update syntax* is the similar to  javascript.
  - e.g.
  ```
  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername"),
    ..user1
  }
  ```
- Tuple Structs
  - Has a struct name, but no variable names.
    - e.g. `struct Color(i32, i32, i32); let black = Color(0,0,0);`
- You can define structs that don't have any fields. These are called *unit-like structs*.
- Ownership of Struct Data
  - Structs can own data for the duration of their existence.
  - It's possible for structs to store references to data owned by something else, but it requires the use of `lifetimes`.

#### 5.2
- You can format `println!` with `{:?}` for debug information or `{:#?}` for pretty print. Both need to add `#[derive(Debug)]` annotation to the struct. *Note*: There are a lot traits that can be used with the `derive` annotation that can add useful behavior to custom types.
- *methods* are like functions, but are defined within the scope of a struct, enum, or trait object and their first parameter is always `self` (which represents the instance of the struct the method is being called on).
  - We dont need to define the type in the method signature because the compiler knows the type from the context.
  - Methods can take ownership or borrow `self` mutable or immutably just like any other parameter.
  - If we wanted to write to the struct using the method we would use `&mut self`.
  - It is rare to use `self` in the method signature. It is usually only used when the methods tranforms `self ` into something new and doesn't want the caller to use the original instance.
- C and C++ use either `.` or `->` for method calling depending on if you are calling the object directly or calling it on an object pointer. Rust has a feature called *automatic referencing and dereferencing*. Rust automatically adds `&`, `&mut`, or `*` so `object` matches the signature of the method.
  - e.g. `pi.distance(&p2);` is the same as `(&p1).distance(&p2);`
- You can define functions in the `impl` block that don't take `self` as a parameter. These are called *associated functions*. These are NOT methods because you don't have an instance of the struct to work with.
  - These are often used for constructors.
  - You call these functions with `::` with the namespace of the struct (e.g. `Rectangle::`)
- You can have multiple `impl` blocks and it still be valid syntax.

### 6.0 Enums and Pattern Matching
- A type where you *enumerate* it's possible variants.
- One of the most useful enums is `Option` which says if values are something or nothing.
- `match` expressions make it easy to run different code for different enum values.
- The `if let` is a concise idiom that lets you handle enums in your code.
- These enums are most similar to *algebraic data types* in functional languages.

#### 6.1 Defining an Enum
- Defined using `enum <name> {}`
  - e.g. 
  ```
  enum IpAddrKind {
    V4,
    V6,
  }
  ```
- Creating instances uses `::` syntax
  - e.g. `let four = IpAddrKind::V4`
- Enums are namespaced so it allows us to write general functions.
  - e.g. `fn route(ip_kind: IpAddrKind) {}`
- Enums can also contain a tuple type
  - e.g. `V4(String)` or `V4(u8, u8, u8, u8)`
- Enums can have different types of values for each type.
  - e.g.
  ```
  enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
  }
  ```
- You can use a `struct` definition as a value type for an enum
  e.g.
  ```
  struct Ipv4Addr {
    // --snip--
  }

  struct Ipv6Addr {
    // --snip--
  }

  enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
  }
  ```
- You can define methods on enums with the `impl` keyword
  - e.g.
  ```
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }
  impl Message {
    fn call(&self) {
      // method body would be defined here
    }
  } 
  ```
- The Option Enum (instead of null)
  - The compiler can check if all cases are handled preventing common bugs in other languages.
  - Defined as:
    ```
    enum Option<T> {
      Some(T),
      None,
    }
    ```
  - It is included in the prelude.
  - The compiler insures that `Option<T>` gets converted to `T` before using the value. This basically automates checks for `None` (null checks in other languages)
  - Everywhere that has a value that is NOT `Option<T>` can be assumed to be a non null.
  - The `match` expression is used with enums to handle each case.

#### 6.2 The match Control Flow Operator
- `match` can compare against literal values,variable names, and wildcards to name a few.
- The compiler confirms all possible cases are handled.
- You can add `{}` to a match arm to execute multiple lines.
- match arms can bind to the parts of the values that match the pattern (used to extract values out of enum variants)
- Patterns in Rust are *exhaustive*; all cases must be covered.
- `_` is the *default* or *catch all* case.
- `()` is the unit value to return so nothing happens.
  - e.g. `_ => ()`

#### 6.3 Concise Control Flow with `if let`
- `if let` is a way to match one pattern while ignoring the rest.
  e.g.
  ```
  let some_u8_value = Some(8);
  if let Some(3) = some_u8_value {
    println!("three");
  }
  ```
  is the same as
  ```
  let some_u8_value = Some(0u8);
  match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
  }
  ```
- Using `if let` over `match` is a trade of conciseness for exhaustiveness.
- You can also use an `else` statement after an `if let`.

## Day 12
### 7.0 Managing Growing Projects with Packages, Crates, and Modules
  - Ways Rust manages code organization
    - *Packages*: A cargo feature that lets you build, test, and share creates.
    - *Crates*: A tree of modules that produces a library or executable.
    - *Modules* and *use*: Let you control the organization, scope, and privacy of paths.
    - *Paths*: A way of naming an item, such as struct, function, or module.

#### 7.1 Packages and Crates
  - The *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate.
  - A package is one or more crates that provide a set of funtionality and contains a *Cargo.toml* file.
  - A package MUST contain **zero or one** library crates and NO more.
  - It can contain **as many** binary crates as you'd like.
  - It must contain **at least one** library or binary crate.
  - `src/main.rs` is the crate root of a binary crate with the same name as the package.
  - `src/lib.rs` is the crate root of a library crate with the same name as the package.
  - Cargo passes the crate root files to rustc to build the library or binary.
  - A package can contain both a `src/main.rs` and a `src/lib.rs` both with the same name as the package.
  - A pacakge can have multiple bnary crates by placing files in the *src/bin*.
  - A crate will group related functionality in a scope (i.e. namespace).

#### 7.2 Defining Modules to Control Scope and Privacy
- `use` brings a path into scope
- `pub` is to make items public
-  Modules let us organize code in a crate into groups. They also control whether the code is *public* or *private*.
- modules use the `mod` keyword
- modules can hold definitions for structs, enums, constants, traits, and functions.
- modules can have/be *children*, *siblings*, or *parents*.
- Modules are like a filesystem directory for your code.

#### 7.3 Paths for Reffering to an item in the Module Tree
- We use a *path* to find an item in a module tree.
- A path can take two forms
  - *absolute path* starts fromo a crate root by using the crate name or a literal `crate`.
  - *relative path* starts from the current module and uses `self`, `super` or an identifier in the current module.
  - both paths are followed by one or more identifiers separated by `::`
- Putting items in a module makes it private
- Everything in Rust is private by default
- A *parent* cannot use items in a *child* module, but a *child* can use all items from its *ancestors*.
- *siblings* can access one anothers private code.
- *super* lets us refer to the the *parent* of a module.
- We can also use `pub` to designate structs and enums as public.
  - `pub` on a struct will make a struct public but leave all the fields private. We can change each of the fields on a case-by-case basis.
  - If your struct has a private field you need a public function to construct it.
  - If an  `enum` is public, all of its variants are public. This is an exception to the *private by default* that is Rust.

#### 7.4 Bringing Paths into Scope with the use Keyword
- `use` brings a path into scope to use as if they were local.
- `use` is similar to a symbolic link in a filesystem
- `use` can also be used with a relative path.
- it is idiomatic to leave the parent module in the scope of `use` for functions
  - e.g. `use crate::front_of_house::hosting;` instead of `use crate::front_of_house::hosting::add_to_waitlist;`
  - it makes it clear that the function isn't locally defined
- However it is idiomatic to specify the full path for structs, enums, and other items
  - The exception is when we bring in two items of the same name from a different scope.
- `as` allows you to alias a path you bring into scope with `use`
  - e.g. `use std::io::Result as IoResult;`
- when we bring something into scope with `use` it is private. We can combine it with `pub` to re-export it.
  - e.g. `pub use crate::front_of_house::hosting`
  - Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. Basically rename your exports for those that are consuming your code (a.k.a) a cleaner api) as opposed to those using your code.
- Add dependencies in the `Cargo.toml`. This tells cargo to download the dependency from [crates.io](https://crates.io/). You then bring them into scope with `use`.
- You can use nested path imports with `{}`
  - e.g. `use std::{cmp::ordering, io};`
  - You can use the `self` in the nesting
    - e.g. `use std::io::{self, Write};` self refers to `use std::io;`
- We can bring all public items into scope with the glob operator. Often used in testing and the prelude.
  - e.g. `use std::collections::*;`

#### 7.5 Separating Modules into Different Files
- When you use a `;` after a `mod` instead of `{}` it tells Rust to load the contents of the module from another file with the same name as the file.

### 8.0 Common Collections
- Stored on the heap
- Each collection has different capabilities and costs.
- Will be discussing 
  - *vector*: Store a variable number of values next to one another
  - *string*: collection of characters
  - *hashmap*: associates a value with a particular key
- You can find more collections [here](https://doc.rust-lang.org/std/collections/index.html).

#### 8.1 Storing Lists of Values with Vectors
- `Vec<T>` is a vector. It allows you to store more than one value of the same type.
- A new vector is created with `let v: Vec<i32> = Vec::new();`
  - Only used when we don't store any values in it and the compiler doesn't know what we intend to store.
- You can also create one the more common way with the vector macro `let v = vec![1,2,3];`
- `mut` makes it mutable.
- When it goes out of scope all the values in the vector go out of scope as well. Including any references you may have to values in the vector.
- You can retrieve values two ways
  - e.g. if `let v = vec![1,2,3,4,5]`, access it with: `let third: &i32 = &v[2];` or `v.get(2)`
  - The first way gives us a reference, the second way gets us `Option<&T>`
  - This means that the second way requires a `match` block to retrieve the value
     - e.g.
     ```
    match v.get(2) {
      Some(third) => println!("The third element is {}", third),
      None => println!("There is no third element."),
    }
    ```
  - If there is not a value when there is a reference, Rust will panic and the program will crash.
  - If there is not a value in the `Option<&T>` case it will return `None`
  - The following code fails because you cannot have a mutable and immutable reference in the same scope. The reason this occurs is because a vector may run out of memory and have to reallocate to add space, which would make the earlier reference to the first element deallocated memory.
  ```
  let mut v = vec![1, 2, 3, 4, 5];
  let first = &v[0];
  v.push(6);
  println!("The first element is: {}", first);
  ```
  - Iteration is done with `for i in &v`
  - We can also iterate over a mutable vector with the following. This is a case where we have to use the dereference operator to get the value.
  ```
  let mut v = vec![100, 32 57];
  for i in &mut v {
    *i += 50;
  }
  ```
  - In order to allow a vector to store multiple types, you can use an enum. See below.
  ```
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
  ```
  - If you don't know the exhaustive set of types the program will get at runtime, then you will have to use a trait object instead of an enum.

## Day 13

#### 8.2 Storing UTF-8 Encoded Text with Strings
- Strings are a collection of bytes
- There is only one string type in the core of the language, a string slice `str` that is usually seen in its borrowed form `&str`
- Both `str` and `String` are UTF-8 encoded.
- The Rust standard library also has `OsString`, `OsStr`, `CString`, and `CStr`. With the latter of each being the borrowed variant.
  - `String` is valid UTF-8 that may contain zeros.
  - `OsString` is not constrained to adhere to Rust strings that are *valid UTF-8 and may contain zeros*
  - `CString` is a nul-terminated string with no nul bytes in the middle.
- created with `let mut s = String::new();`
- The `to_string` creates a `String`, the method is on any type that implements the `Display` trait; which string literals do.
- `String::from()` can also be used to create a `String`
- You can use the `+` operator or the `format!` macro to concatenate `String` values
- The `.push_str()` method is used to append a string slice.
- The `.push()` method appends a single character.
- Rust strings do not support indexing.
- A `String` is wrapped over a `Vec<u8>.`
- There are three ways to look at strings in Rust, as bytes, scalar values (characters), or grapheme clusters.
- You can create a string slice by referencing a range of bytes of a string.
  - e.g. `&mystring[0..4]`
  - If you try to create a string slice with bytes inbetween characters, Rust will panic at runtime as if an invalid index were accessed in a vector.
- You can iterate over a string with `.chars()` and `.bytes()`
  - Remember that valid unicode scalar values may be made up of more than one byte.

#### 8.3 Storing Keys with Associated Values in Hash Maps
- Has the signature  `HashMap<K,V>`
- create with the `HashMap::new()`
- You add new elements with `.insert()`
- Not included in the prelude so it must be explicityly brought into scope from collections
- You can also construct a hashmap by using iterators and the `collect` method on a vector of tuples.
  - e.g. `zip` combines two vectors into a vector of tuples and `collect` turns it into a `Hashmap`. It's possible to `collect` into multiple different data types so `HashMap<_,_>` is used. The underscores allow the compiler to infer the types.
  ```
  fn main() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
      teams.into_iter().zip(initial_scores.into_iter()).collect();
  }
  ```
- Types that have the `Copy` trait are copied into the hashmap, where owned values are moved and the hash map is the new owner. This means we won't be able to use the values after they have been *moved* into a hashmap with `insert`.
- If we enter references to values into the hash map, the values won't be moved. But the values that the references point to, must be valid for *at least* as long as the hash map is valid.
- Values can be retrieved with the `get` method. `get` returns an `Option<&V>`
- You can iterate with them over `for (key, value) in &myMap{}`
- *Overwrite*: By default `insert` will overwrite the previous key.
- *Insert if key has no value*: `entry` takes a key you want to insert and gives you an `Entry` enum that represents a value that may or may not exist. `or_insert` on `Entry` is defined to return a mutable reference to the value for the `Entry` key if it exists. Otherwise it enters the new value provided.
- *Update on*: Same as previous see, below example
```
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
    }

    println!("{:?}", map);
}
```
- The default hash function isn't the fastest as it is DoS resistant. You can replace it if needed by specifying a different *hasher* (a type that implmements `BuildHasher`). [crates.io](https://crates.io/) has libraries with common hashing algorithms.

## Day 14
*../examples/chapter8_exercises/\**

## Day 15

### 8.0
- There are two types of errors in Rust, *recoverable* and *unrecoverable*. 
- Rust doesn't have exceptions, it has `Result<T,E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters and unrecoverable error.

#### 8.1 Unrecoverable Errors with `panic!`
- When the `panic!` macro executes, your program prints afailure message, unwinds and cleans the stack, and then quits.
  - *unwinding* means Rust walks back up the stack and cleans up the data from each function it encounters.
  - *unwinding* is expensive and you can have your program not clean up memory (let the OS do it) by using `panic = 'abort'` to the appropriate  `[profile]` sections in your `Cargo.toml` file.
- We can get more detailed information (backtrace) about inner `panic!`s by setting the `RUST_BACKTRACE=1` environment variable. Detailed backtraces need to have debug symbols enabled (which is default with `cargo build` or `cargo run` without the `--release` flag)

#### 8.2 Recoverable Errors with Result
- The `Result` enum has two variants `Ok` and `Err`
```
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```
- You can use `match` to handle the result, like so
```
let f = File::open("hello.txt");

let f = match f {
  Ok(file) => file,
  Err(error) => panic!("Problem opening the file: {:?}", error);
};
```
- `Result` is brought into scope in the prelude like `Option`.
- We can handle differet errors differently with embedded matches
```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f  = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      }
    }
  }
}
```
The same code written with closures would look like:
```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem createing the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  })
}
```
- `.unwrap()` is a shortcut expression implemented like the `match` arms.
  - It will either return the `Ok` variant or call the `panic!` macro
- `.expect()` lets us choose the `panic!` error message.
- *Propgating errors* is done by `return Err(e)` where `e` is an error in a `match`, that is `Err(e)`
  - explicit `returns` are not needed if it is the last expression in a function.
  - Propgating is common so it is handled with the `?` operator. This operator works *almost* same as the previous  `match` expressions.
  - The difference is that error values with `?` get passed through the `from` function defined by the `From` trait. This is used to convert one type of error to another.
  - This allows you to chain together function calls like below
    - e.g.
    ```
    use std::fs::File;
    use std::io;
    use std::io::Read;
    fn read_username_from_file() -> Result<String, io::Error> {
      let mut s = String::new();
      File::open("hello.txt")?.read_to_string(&mut s)?;
      Ok(s)
    }
    ```
    which eventually turns into:
    ```
    use std::fs;
    use std::io;
    fn read_username_from_file() -> Result<String, io::Error> {
      fs::read_to_string("hello.txt")
    }
    ```
  - The `?` operator can be used in functions that return type `Result` (or `Option`) because it works the same way as `match`
- The main function has a signature of `fn main() -> Result<(), Box<dyn Error>> { } ` where `Box<dyn Error>` basically means "any kind of error".

#### 9.3 To `panic!` or Not to `panic!`
- `unwrap` and `expect` are great placeholders for unhandled errors. 
- `unwrap` is also appropriate if you have more information that the compiler and can ensure `Result` will have an `Ok`.
  - e.g.
  ```
  use std::net::IpAddr;
  let home: IpAddr = "127.0.0.1".parse().unwrap();
  ```
- You should panic on values that are not valid.
- Functions often have contracts: their behavior is only guaranteed if the inputs meet particular requirements. Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug and it’s not a kind of error you want the calling code to have to explicitly handle. In fact, there’s no reasonable way for calling code to recover; the calling programmers need to fix the code. Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function.
- You could use types to ensure safety of values
  - e.g. Overwrite the constructor `new()`
  ```
  pub struct Guess {
    value: i32,
  }

  impl Guess {
    pub fn new(v: i32) -> Guess {
      if value < 1 || value > 100 {
        panic!("Guess value must be between 1 and 100, got {}", v);
      }
      Guess { value }
    }
    pub fn value(&self) -> i32 {
      self.value
    }
  }
  ```
### 10.0 Generic Types, Traits, and Lifetimes
- Generics reduce code duplication, Traits define generic behavior, Lifetimes allow borrowing values while still letting the compiler check for valid references

#### 10.1 Generic Data Types
- Type naming is CamelCased and done in `<>`
  - e.g. `fn largest<T>(list: &[T]) -> &T { }`
    - This is read as the function `largest` is generic over some type `T`.
- Generics can be used in Structs
  - e.g. `struct Point<T> {x: T, y: T,}`
  - e.g. with same or different types `struct Point<T, U> {x: T, y: U,}`
- Generics in Enums
  - e.g. `enum Option<T> {Some(T), None}`
  - e.g. with same or different types `enum Result<T,E> { Ok(T), Err(E),}`
-  Generics in method definitions
  - e.g. from above `Point` struct
  ```
  impl<T> Point<T> {
    fn x(&self) -> &T {
      &self.x
    }
  }
  ```
- You can also declare specified methods inside of a generic struct that will only exist on instead of `Point<f32>` and not any other types that `T` could be.
  - e.g. `impl Point<f32> { }`
  - This can really allow you do overload functions to handle different types
- Further, you can have method signatures with different types than what are in the original struct.
  - e.g. Using the original `Point<T, U> {x: T, y: U,}` 
  ```
  impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
      Point {
        x: self.x,
        y: other.y,
      }
    }
  }

  fn main(){
    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    // p3.x = 5, p3.y = 'c'
    // p1 and p2 are dead (went out of scope in mixup)
  }
  ```
  - `<T,U>` is on the `impl` block because it goes with the struct, and `<V,W>` is with the `fn` block because they are only relevant to the method.
- There is no runtime cost for using generics because Rust uses *monomorphization*. That is, it fills in concrete types at compile time. This is also why Rust has a slower compile time.

#### 10.2 Traits: Defining Shared Behavior
- Shared functionality between different types in an abstract way
- *Traits are similar to interfaces*
- It is declared using the `trait` keyword.
  - e.g. `pub trait Summary {fn summarize(&self) -> String;}`
- It is used by stating the `impl` keyword the trait name then the `for` keyword and the struct name. Format `impl Trait for Struct`.
  - e.g. `impl Summary for NewsArticle`, see below for full example
  ```
  pub trait Summary {
    fn summarize(&self) -> String;
  }

  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }

  impl Summary for NewsArticle {
    fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
  }
  ```
- In order to use the trait, you must first bring it into scope with the `use` keyword. It would also have to be a public trait, i.e. use `pub`
- You cannot implement external traits on external types. **i.e. you cannot alter external structs functionality (like you can override default functionality in javascript)**. This is known as *coherence*, a.k.a the *orphan rule*.
  - Without this rule, two crates could implement the same trait for the same type and Rust wouldn't know which implementation to use.
- Traits can have a default implementation
  - To use this default we keep an empty `impl` block like so `{}`.
    - e.g. `impl Summary for NewsArticle {}`
  - Default implementations can call other methods of a trait that do not have a default implementation.
- We can use traits as parameters by using `&impl` and trait name
  - e.g. `pub fn method_name(var: &impl Trait)`, full example below
  ```
  pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
  }
  ```
  - We can call any method on the trait. If we try to pass in other variables, it will not compile.
- `impl Trait` syntax works for straightforward cases, but is syntax sugar for *trait bound* syntax. The below is the same as: `pub fn notify(item: &impl Summary) { }`
  - e.g. `pub fn notify<T: Summary>(item: &T){ }`
- If we wanted to have any type that implements a trait we could use ` ub fn notify(item: &impl Summary) { }`
- If we wanted to ensure they were the same type we would use *trait bound* syntax: `pub fn notify<T: Summary>(item1: &T, item2: &T) { }`
- We can specify multiple *trait bound* parameters with `+`
  - e.g. `pub fn notify(item: &(impl Summary + Display)) { }`
  - e.g. It also works with generics `pub fn notify<T: Summary + Display>(item: &T) { }`
- An alternate way to express *trait bound* parameters are with the `where` clause. The following are equivalent
  - e.g. `fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { }`
  - e.g. 
  ```
  fn some_function<T, U>(t: &T, u: &U) -> i32 
    where T: Display + Clone, 
          U: Clone + Debug 
  { }
  ```
- You can also return types that implement traits with `-> impl Trait`
  - e.g. `fn returns_summarizable() -> impl Summary { }`
  - This is especially useful in the context of closures and iterators
  - One caveat, you cannot return multiple types with the `impl Trait` syntax due to how it works. See [Chapter 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits) if this is confusing. However it will be covered later how to do this, since apparently there is a way.
- You can use *Trait Bounds* with `impl` to condionally implement Trait methods.
  - e.g.
  ```
  use std::fmt::Display;

  struct Pair<T> {
    x: T,
    y: T,
  }

  impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
      Self { x, y }
    }
  }

  impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
      if self.x >= self.y {
        println!("The largest member is x = {}", self.x);
      } else {
        println!("The largest member is y = {}", self.y);
      }
    }
  }
  ```
- We can also conditionally implement a trait for any type that implements another trait. These are called *blanket implementations*
  - e.g. 
  ```
  impl<T: Display> ToString for T {
    // --snip--
  }
  ```
- Summary
  - Trait bound syntax
    - e.g. `function<T: Trait1 + Trait2>(item: &T)`
    - Shorthand (can only use 1 trait): `function(item: &impl Trait)`
  - Returning traits
    - e.g. `-> impl Trait`
  - You implement traits on structs
    - e.g. `impl Trait for Struct`
  - You can implment conditional trait bounded struct methods
    - e.g. `impl <T: Trait1 + Trait2> Struct<T>`
  - You can implement traits on trait bounded generics
    - e.g. `impl<T: Trait1> NewTrait for T`

## Day 16

#### Validating References with Lifetimes
- Most of the time they are inferred like types. Lifetimes only need to be annotated when they could be related in a multiple ways. This is done using generic lifetime parameters
- Lifetimes prevent dangling references with the borrow checker
- Lifetime annotations don't change how long the references live.
- *Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.*
- Lifetime examples
```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&a' mut i32 // a mutable reference with an explicit lifetime
```
- **Lifetime annotations only have meaning when they have a relation to another lifetime.**
- Declared in `<>` in between the function name and parameter list.
  e.g. `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { }`
- When you a generic lifetime is substituted, it takes the smallest of the overlapping scopes to be the concrete scope. Refer to [the borrow checker in 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-borrow-checker) for a visual of overlapping scopes.
- structs can hold references, but they need a lifetime value.
  - e.g. `struct Important<'a> {part: &'a str,}`
- The patterns programmed into Rust's compiler analysis are called *lifetime elision rules*
- lifetimes on function methods are called *input lifetimes*, lifetimes on return values are called *output lifetimes*.
- The compiler has three rules, one on input lifetimes and two on output lifetimes.
  1. Each parameter that is a reference gets its own lifetime parameter.
    - e.g. `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`
  2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
    - e.g. `fn foo<'a>(x: &'a i32) -> &'a i32`
  3. If their are multiple input lifetime parameters, but one of them is `&self` and `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.
- All string literals have the `'static` lifetime
  - Suggestions to use `'static` in error messages can often caused by a dangling reference, or a mismatch of available lifetimes.
- Lifetimes are a type of generic and share space in the `<>`


  
# Things I am struggling with and need to review
- Chapter 4 [String Slices as Parameters](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices-as-parameters)
- Chapter 10 [Using Trait Bounds to Conditionally Implement Methods](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)


# Resources
- [Foundational Distributed Systems Papers](https://muratbuffalo.blogspot.com/2021/02/foundational-distributed-systems-papers.html?m=1)
- [99% Fault Tolerence](https://vitalik.ca/general/2018/08/07/99_fault_tolerant.html)
