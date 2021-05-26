## Day 15

### 9.0
- There are two types of errors in Rust, *recoverable* and *unrecoverable*. 
- Rust doesn't have exceptions, it has `Result<T,E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters and unrecoverable error.

#### 9.1 Unrecoverable Errors with `panic!`
- When the `panic!` macro executes, your program prints afailure message, unwinds and cleans the stack, and then quits.
  - *unwinding* means Rust walks back up the stack and cleans up the data from each function it encounters.
  - *unwinding* is expensive and you can have your program not clean up memory (let the OS do it) by using `panic = 'abort'` to the appropriate  `[profile]` sections in your `Cargo.toml` file.
- We can get more detailed information (backtrace) about inner `panic!`s by setting the `RUST_BACKTRACE=1` environment variable. Detailed backtraces need to have debug symbols enabled (which is default with `cargo build` or `cargo run` without the `--release` flag)

#### 9.2 Recoverable Errors with Result
- The `Result` enum has two variants `Ok` and `Err`
```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```
- You can use `match` to handle the result, like so
```rust
let f = File::open("hello.txt");

let f = match f {
  Ok(file) => file,
  Err(error) => panic!("Problem opening the file: {:?}", error);
};
```
- `Result` is brought into scope in the prelude like `Option`.
- We can handle differet errors differently with embedded matches
```rust
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
```rust
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
    ```rust
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
    ```rust
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
  ```rust
  use std::net::IpAddr;
  let home: IpAddr = "127.0.0.1".parse().unwrap();
  ```
- You should panic on values that are not valid.
- Functions often have contracts: their behavior is only guaranteed if the inputs meet particular requirements. Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug and it’s not a kind of error you want the calling code to have to explicitly handle. In fact, there’s no reasonable way for calling code to recover; the calling programmers need to fix the code. Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function.
- You could use types to ensure safety of values
  - e.g. Overwrite the constructor `new()`
  ```rust
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
  ```rust
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
  ```rust
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
  ```rust
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
  ```rust
  pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
  }
  ```
  - We can call any method on the trait. If we try to pass in other variables, it will not compile.
- `impl Trait` syntax works for straightforward cases, but is syntax sugar for *trait bound* syntax. The below is the same as: `pub fn notify(item: &impl Summary) { }`
  - e.g. `pub fn notify<T: Summary>(item: &T){ }`
- If we wanted to have any type that implements a trait we could use `pub fn notify(item: &impl Summary) { }`
- If we wanted to ensure they were the same type we would use *trait bound* syntax: `pub fn notify<T: Summary>(item1: &T, item2: &T) { }`
- We can specify multiple *trait bound* parameters with `+`
  - e.g. `pub fn notify(item: &(impl Summary + Display)) { }`
  - e.g. It also works with generics `pub fn notify<T: Summary + Display>(item: &T) { }`
- An alternate way to express *trait bound* parameters are with the `where` clause. The following are equivalent
  - e.g. `fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { }`
  - e.g. 
  ```rust
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
  ```rust
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
  ```rust
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