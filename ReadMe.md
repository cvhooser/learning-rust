# Starting Rust
## Table of Contents
**Note:** *Open in [Obsidian](https://obsidian.md/) for best viewing experience.*

[[Day 1]] - IDE, formatting, and compiling
[[Day 2]] - Debugger Configuration
[[Day 3]] **ch 2** - Guessing Game
[[Day 4, 5, 6]] - Technical Readings
[[Day 7]] **ch 3** - Variables and Primitives
[[Day 8]] **ch 3 | ch 4** - Functions, Control Flow, and Ownership
[[Day 9]] **ch 4** - References, Borrowing, and Slice Type
[[Day 10, 11]] **ch 5 | ch 6** - Structs, Enums, and Pattern Matching
[[Day 12]] **ch 7 | ch 8** - Crate and Vectors
[[Day 13]] **ch 8** - Strings and Hashmaps
[[Day 14]] - Exercises
[[Day 15]] **ch 9 | ch 10** - Error Handling, Generic Types, and Traits
[[Day 16]] **ch10** - Lifetimes
[[Day 17]] **ch 11** - Testing
[[Day 18]] **ch 12** - Building a Command Line Program
[[Day 19]] **ch 13** - Closures and Iterators
[[Day 20]] **ch 14** - Cargo and Crate
[[Day 21]] **ch 15** - Smart Pointers: `Box<T>` and `Rc<T>`
[[Day 22]] **ch 15 | ch 16** - Smart Pointers: `RefCell<T>` Concurrency: Threads, and Message Passing
[[Day 23]] **ch16 | ch 17** - Shared State, Sync / Send Traits, and Object Oriented Features (Trait Objects)
[[Day 24]] **ch 17** - Blog example
[[Day 25]] **ch 18** - Pattern Matching
[[Day 26]] **ch 19** - Unsafe Rust
[[Day 27]] **ch 19** - Advanced: Traits, Types, Functions and Closures. Also Macros
[[Day 28]] **ch20**
[[Day 29]] **Appendix Notes**
- [Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
- [Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html)
- [Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
- [Useful Development Tools](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html)

## Strategy to breakthrough
- RUST
	- [x] I'm going to go through [The Rust Book](https://doc.rust-lang.org/book) in its entirety.
	- [ ] Then I plan to move on to [rustlings](https://github.com/rust-lang/rustlings) and see how challenging they are. 
	- [ ] There is also [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/) that is probably worth running through.
	- [ ] After that I should have enough of an understanding to be able to start combing through open source projects code bases (polkadot) and make some bug fixes.
	- [ ] Then I want to attempt to write an implementation of the [RAFT](https://raft.github.io).
	- [ ] At this point I should have enough of an understanding to start applying for jobs.
- Distributed Systems
	- [ ] Starting from the beginning, read and note my way through these [Foundational Distributed Systems Papers](https://muratbuffalo.blogspot.com/2021/02/foundational-distributed-systems-papers.html?m=1).
	- [ ] Research and take notes on other consensus algorithms
- Blockchain
	- [ ] Research and understand the technicals of Polkadot.
	- [ ] Become an active member and contributor on the polkadot discord server
	- [ ] Write a stellar sdk in Rust

# Things I am struggling with and need to review
- Chapter 4 [String Slices as Parameters](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices-as-parameters)
- Chapter 10 [Using Trait Bounds to Conditionally Implement Methods](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)
- Chapter 17 [Generics vs. Trait Objects and when to use them](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)


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
- [Rust Books](https://github.com/sger/RustBooks)
- [Foundational Distributed Systems Papers](https://muratbuffalo.blogspot.com/2021/02/foundational-distributed-systems-papers.html?m=1)
- [99% Fault Tolerence](https://vitalik.ca/general/2018/08/07/99_fault_tolerant.html)
- [Async discussions](https://users.rust-lang.org/t/does-rust-need-rx-implementation-and-or-more/29481/7)
- [Random Blog](https://www.logicalshift.io/)

# Useful Crates
- [Sodium](https://github.com/SodiumFRP/sodium-rust): A Functional Reactive Programming (FRP) library for Rust
- [Signals](https://github.com/Pauan/rust-signals): This is a Rust crate that provides zero-cost Signals which are built on top of the [futures](https://crates.io/crates/futures) crate.
- [Desync](https://github.com/Logicalshift/desync): A concurrency library for Rust that protects data by scheduling operations in order instead of locking and blocking threads. [docs](https://docs.rs/desync/0.7.0/desync/) [article](https://www.logicalshift.io/articles/rust-tools/desync.html)
- [Flo_binding](https://docs.rs/flo_binding/2.0.0/flo_binding/): A library of types to help store state in interactive applications.