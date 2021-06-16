# Rust By Example
## Comments
`/// library docs for the following item`
`//! library docs for the enclosing item`
`cargo test --doc` runs document test cases
`#[doc(inline)]` is used to inline docs
`#[doc(no_inline)]` is used to prevent linking out to a separate page
`[doc(hidden)]` using this tells `rustdoc` not to include this in documentation
[library docs](https://doc.rust-lang.org/rust-by-example/meta/doc.html)
`eprintln!` prints to `io::stderr`
`fmt::Debug` uses `{:?}` marker
`fmt:Display` uses `{}` marker
`{<number>}` and `{<number>:?}` allow you to order inputs, with regular or debug
- e.g. `println!("Item0 {0}, Item1 {1}, Item2: {2:?}", Zero, One, Two)`
`{named:?}` can print named debug attributes
- e.g. `println!("My name is {name:?}", name="Cory")`
`{:#?}` pretty prints things
