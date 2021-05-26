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
     ```rust
    match v.get(2) {
      Some(third) => println!("The third element is {}", third),
      None => println!("There is no third element."),
    }
    ```
  - If there is not a value when there is a reference, Rust will panic and the program will crash.
  - If there is not a value in the `Option<&T>` case it will return `None`
  - The following code fails because you cannot have a mutable and immutable reference in the same scope. The reason this occurs is because a vector may run out of memory and have to reallocate to add space, which would make the earlier reference to the first element deallocated memory.
  ```rust
  let mut v = vec![1, 2, 3, 4, 5];
  let first = &v[0];
  v.push(6);
  println!("The first element is: {}", first);
  ```
  - Iteration is done with `for i in &v`
  - We can also iterate over a mutable vector with the following. This is a case where we have to use the dereference operator to get the value.
  ```rust
  let mut v = vec![100, 32 57];
  for i in &mut v {
    *i += 50;
  }
  ```
  - In order to allow a vector to store multiple types, you can use an enum. See below.
  ```rust
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