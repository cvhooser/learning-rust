## Day 19

### 13.0 Functional Language Features: Iterators and Closures
- *closures*, a function-like construct you can store in a variables
- *iterators*, a way of processing a series of elements
- Both of these are faster than you would think and required for writing idiomatic Rust

#### 13.1 Closures: Anonymous Functions that Can Capture Their Environment
- closure definition is `|<var>| { }` 
- Does not require type declarations, because of type inference, but it can.
- You cannot call a closure with different types of variables
- encapsulated caching is also called *memorization* or *lazy evaluation*
- `Fn` is a trait
	- All closures implement `Fn`, `FnMut`, `FnOnce`
	- Functions can implement all three of the `Fn` traits too. If what we want to do doesn’t require capturing a value from the environment, we can use a function rather than a closure where we need something that implements an `Fn` trait.
- Closures capture their environment and access variables from the scope in which they were defined.
- When closures capture a value from the environment it uses memory to store the values for use in the body; which incurs overhead and is done in three different ways.
	- `FnOnce` consumes the variables from ithe enclosing scope (i.e. environment). Since the closure needs to take ownership from the enclosing scope, it can only be called once.
	- `FnMut` can change the environment because it mutably borrows values
	- `Fn` borrows values from the environment immutably
	- **Rust infers the trait  based on how the closure uses the values from the environment**
		- All closures implmement `FnOnce`
		- Closures that don't move the captured variables also implement `FnMut`
		- Closures that don't need mutable access also implement `Fn`
- The `move` keyword can force the closure to take ownership of the values it uses in the environment.
	-  e.g. 
	```rust
		let x = vec![1, 2, 3];
		let equal_to_x = move |z| z == x;
		println!("can't use x here: {:?}", x);
	```
	- Note: `move` closures may still implement `Fn` or `FnMut`, even though they capture variables by move. This is because the traits implemented by a closure type are determined by what the closure does with captured values, not how it captures them. The `move` keyword only specifies the latter.
	- Most of the time when specifying one of the `Fn` trait bounds, you can start with the `Fn` and the compiler will tell you if you need `FnMut` or `FnOnce` based on what happens in the closure body.

#### 13.2 Processing a Series of Items with Iterators
- Iterators are *lazy* i.e. they have no effect until you call methods that consume the iterator to use it up
- All iterators implement a trait names `Iterator`
	- Iterator Trait
	```rust
	pub trait Iterator {
	  type Item;
	  fn next(&mut self) -> Option<Self::Item>;
	}
	```
- Iterators return `Some(v)` and when it is done it returns `None`
- `into_iter` will return owned values instead of the immutable values that are returned from `iter`
- Iterator trait has other methods known as *iterator adaptors* that allow you to change iterators into different kinds of iterators.

#### 13.3 Improving Out I/O Project

#### 13.4 Comparing Performance: Loops vs. Iterators
- Iterators have a zero cost overhead and imposes no additional runtime overhead. 
	- From the original designer and implementer of C++ *In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.*