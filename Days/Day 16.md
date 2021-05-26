## Day 16

#### 10.3 Validating References with Lifetimes
- Most of the time they are inferred like types. Lifetimes only need to be annotated when they could be related in a multiple ways. This is done using generic lifetime parameters
- Lifetimes prevent dangling references with the borrow checker
- Lifetime annotations don't change how long the references live.
- *Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.*
- Lifetime examples
```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
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