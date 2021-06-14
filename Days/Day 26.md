## Day 26

### 19.0 Advanced Features
#### 19.1 Unsafe Rust
- Unsafe Rust is used with the `unsafe` keyword and starts a block where you can perform five actions (a.k.a _unsafe superpowers_)
	1. Dereference a raw pointer
	2. Call an unsafe function or method
	3. Access or modify a mutable static variable
	4. Implement an unsafe trait
	5. Access fields of `union` S
- `unsafe` does not turn off the borrow checker or disable safety checks, it just allows you to perfrom the aforementioned five actions.
	- This tells us that errors related to memory safety will happen inside of an `unsafe` block
- It is recommended to write `unsafe` code in a *safe* code abstraction.

##### Dereferencing a Raw Pointer
- `unsafe` Rust has two new types called *raw pointers*, `*const T` and `*mut T`
	- Note: The `*` is not the dereference operator, it is part of the type name
		- In the context of raw pointers, immutable means that the pointer can't be directly assigned to after being dereferenced.
	- Raw pointers have the following differences from smart pointers
		1. Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
		2. Aren't guaranteed to point to valid memory
		3. Are allowed to be null
		4. Don't implement any automatic cleanup
	- You can create raw pointers by casting them
		- *Note: we don't need an `unsafe` block to create raw pointers, only to dereference them*
		```
		let mut num = 5;
		
		let r1 = &num as *const i32;		
		let r2 - &mut num as *mut i32;
		
		unsafe {
			println!("r1 is: {}", *r1);
			println!("r2 is: {}", *r2);
		}
		```
- One of the biggest reasons to use raw pointers is for call *C* code

##### Calling an Unsafe Function or Method
- Calling an `unsafe` function must be done from an `unsafe` block
	- `unsafe fn dangerous() {}` must be called with `unsafe { dangerous(); }`


##### Creating a Safe Abstraction over Unsafe Code
- Best to look [here](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#creating-a-safe-abstraction-over-unsafe-code) for the best explanation.
	- The example is taking two mutable pointers, one of each half of a string slice. However Rust does not allow you to have two mutable references to the same object, even though in this case it is safe.
	- Code below:
	```
	use std::slice;
	
	fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
		let len = slice.len();
		let ptr = slice.as_mut_ptr();

		assert!(mid <= len);

		unsafe {
			(
				slice::from_raw_parts_mut(ptr, mid),
				slice::from_raw_parts_mut(ptr.add(mid), len - mid),
			)
		}
	}

	fn main() {
		let mut vector = vec![1, 2, 3, 4, 5, 6];
		let (left, right) = split_at_mut(&mut vector, 3);
	}

	```
	
	
##### Using `extern` Functions to Call External Code
- `extern` facilitates the creation and use of a *Foreign Function Interface (FFI)*.
	- An FFI is a way for a programming language to define functions and enable a difference programming language to call those functions
	- Below is an example of how to setup an integration with `abs` function from the C standard library. `extern` blocks are always `unsafe` to call from Rust code
	```
	extern "C" {
		fn abs(input: i32) -> i32;
	}
	
	fn main() {
		unsafe {
			println!("Absolute value of -3 according to C: {}", abs(-3));
		}
	}
	```
- Within an `extern` block we list the names and signatures of the external functions from the language we want to call. The `"C"` part defines an *application binary interface (ABI)* the external function uses and defines how to call the function at the assembly level. `"C"` is the most common.
- We can use the `extern` keyword to allow other languages to call Rust functions.
	- e.g.
	```
	#[no_mangle]
	pub extern "C" fn call_from_c() {
		println!("Just called a Rust function from C!");
	}
	```
	-  `#[no_mangle]` tells the Rust compiler not to *mangle* the function name.
		-  *Mangling* is when a compiler changes the name we've given a function to a different name containing more information for other parts of the compilation process.


##### Accessing or Modifying a Mutable Static Variable
- global variables are called `static` variables in rust and are in the `SCREAMING_SNAKE_CASE` and must be type annotated.
	- e.g. `static HELLO_WORLD: &str = "Hello, world!";`
- `static` variables have a `static` lifetime and a fixed address in memory.
- `static` varaibles can be mutable, Constants cannot
	- Accessing a mutable `static` variable is `unsafe`.
	- e.g.
	```
	static mut COUNTER: u32 = 0;

	fn add_to_count(inc: u32) {
		unsafe {
			COUNTER += inc;
		}
	}

	fn main() {
		add_to_count(3);

		unsafe {
			println!("COUNTER: {}", COUNTER);
		}
	}
	```
	

##### Implementing an Unsafe Trait
- A trait is unsafe when at least one of its methods has some invariant that the compiler can't verify.
```
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}

```
- As an example, recall the `Sync` and `Send` marker traits we discussed in the [“Extensible Concurrency with the `Sync` and `Send` Traits”](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html#extensible-concurrency-with-the-sync-and-send-traits) section in Chapter 16: the compiler implements these traits automatically if our types are composed entirely of `Send` and `Sync` types. If we implement a type that contains a type that is not `Send` or `Sync`, such as raw pointers, and we want to mark that type as `Send` or `Sync`, we must use `unsafe`. Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads; therefore, we need to do those checks manually and indicate as such with `unsafe`.

##### Accessing Fields of a Union
- A `union` is similar to a `struct`, but only one declared field is used in a particular instance at one time. Unions are primarily used to interface with unions in C code.
- It is unsafe because Rust can't guarantee the type of data currently being stored in the union instance.