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
      ```rust
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