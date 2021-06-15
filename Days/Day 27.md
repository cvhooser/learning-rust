## Day 27

#### 19.2 Advanced Traits
- *Associated types* connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.
	- e.g.
	```
	pub trait Iterator {
		type Item;

		fn next(&mut self) -> Option<Self::Item>;
	}

	```
	- Implementers of the `Iterator` will specify the concrete type of `Item`
	- If we were to implement `Iterator` with generics instead of an associated type, then we would have to provide type annotations to indicate which implementation of Iterator we want to use.
	- Basically how Rust implements abstract types with default implementations
- You can overload operators in the `std::ops` library
	- e.g. `std::ops::Add` will overload the `+` operator. There is an associated type `type Output` that you can set in the `impl` to return the type you want when you overload the operator. See [here](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#default-generic-type-parameters-and-operator-overloading)
- The add operator has a default generic with the syntax `<PlaceholderType=ConcreteType>`
	- e.g.
	```
	trait Add<Rhs=Self> {
		type Output;

		fn add(self, rhs: Rhs) -> Self::Output;
	}

	```
- Here is an example where you can add together a custom `Rhs`
	```
	use std::ops::Add;

	struct Millimeters(u32);
	struct Meters(u32);

	impl Add<Meters> for Millimeters {
		type Output = Millimeters;

		fn add(self, other: Meters) -> Millimeters {
			Millimeters(self.0 + (other.0 * 1000))
		}
	}

	```
- Default type parameters are used in two ways:
	1. To extend a type without breaking existing code
	2. To allow customization in specific cases most users won't need
- You can implement multiple traits on the same type, that share a method name. But you have to call them differently.
	- e.g. `Type::method(&var);`
	```
	trait Pilot {
    	fn fly(&self);
	}

	trait Wizard {
		fn fly(&self);
	}

	struct Human;

	impl Pilot for Human {
		fn fly(&self) {
			println!("This is your captain speaking.");
		}
	}

	impl Wizard for Human {
		fn fly(&self) {
			println!("Up!");
		}
	}

	impl Human {
		fn fly(&self) {
			println!("*waving arms furiously*");
		}
	}

	fn main() {
		let person = Human; 
		Pilot::fly(&person); 
		Wizard::fly(&person); 
		person.fly();
	}

	```
- When using associated functions that are part of traits there is not a `self` to disambiguate which function to call. This means you have to use _fully qualified syntax_ `<Type as Trait>::function(receiver_if_method, next_arg, ...);`
	- e.g. `<Dog as Animal>::run());` vs. `Dog::run();`
	- *Note*: All functions can be called this way, it is just rare that Rust cannot infer some amount of the information.

##### SuperTraits
- Having one trait use another trait's functionality. Form: `trait MyTrait: some_lib:: OtherTrait {}`
	- This allows us to use the functions that a `OtherTrait` has. See [here](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-supertraits-to-require-one-traits-functionality-within-another-trait) if there is confusion.

##### Newtype Pattern
- It allows us to wrap types in a tuple to get around not being able to implement external traits on external types
	- e.g. As an example, let’s say we want to implement `Display` on `Vec<T>`, which the orphan rule prevents us from doing directly because the `Display` trait and the `Vec<T>` type are defined outside our crate. We can make a `Wrapper` struct that holds an instance of `Vec<T>`; then we can implement `Display` on `Wrapper` and use the `Vec<T>` value, as shown in Listing 19-23.
	```
	use std::fmt;

	struct Wrapper(Vec<String>);

	impl fmt::Display for Wrapper {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "[{}]", self.0.join(", "))
		}
	}

	fn main() {
		let w = Wrapper(vec![String::from("hello"), String::from("world")]);
		println!("w = {}", w);
	}

	```
- If we want all of the types of the inner object for the wrapper to use, then a solution would be to implement the `Deref` trait 


#### 19.3 Advanced Types
- We can use types to alias othertypes, almost like enums
	- e.g. `type Kilometers = i32;`
- The best use case for aliases is to reduce having to write out long types
	- e.g. So much cleaner!
	```
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| ())
    }

	```
- The above can also be done with generics, like in the `std::io` crate: `type Result<T> = std::result::Result<T, std::io::Error>;`
- The _Never Type_ is `!` and it represents the return time when a function will never return.
	- This operator can have allow you to coerce returns into any type. e.g. having different returns from a match arm.
- Rust uses the `Sized` trait to determine whether or not a size is known at compile time. By default Rust adds `Sized` to all declared generics
	- You can relax this constrait by using `fn generic<T: ?Sized\>(t: &T) {}`. _Note the use of `t: &T`. This is because we need it to be a pointer since the type may not be sized_

#### 19.4 Advanced Functions and CLosures
- The `fn` type is a *function pointer*.
	- e.g. `do_twice(func: fn(i32) -> i32, arg: i32) -> i32 { func(arg) + func(arg)}`
- Function pointers implement all three closure traits (`Fn`, `FnMut`, and `FnOnce`)
- An exmplae of where you only want to accept `fn` and not closures is when you are interfacing with external code that doesn't have closures, i.e. C.
- `()` is an intializer that are implemented as functions returning an instance that is constructed from their arguments. We can use these as function pointers that implement that closure traits
	- e.g.
	```
	enum Status {
		Value(u32),
		Stop,
	}

	let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

	```
- You cannot usre functions as a return type, but you can return a `Boxed<dyn Fn(i32) -> i32>`

#### 19.5 Macros
- The term *macro* refers to a family of features in Rust: *declarative* macros with `macro_rules!` and three kinds of *procedural* macros
	1. Custom `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums
	2. Attribute-like macros that define custom attributes usable on any item
	3. Function-like macros that look like function calls but operate on the tokens specified as their argument.
- Macros can take a variable number of parameters
- Macros are expanded before the compiler interprets the meaning of the code.
	- This means a macro can implement a trait on a given type. A function can't because it gets called at runtime and a trait needs to be implemented at compile time.
- A macro must be defined and brought into scope *before* you call them in a file. Where functions order does not matter.
- Lets look at a barebones definition of the `vec!` macro
```
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

```
- `#[macro_export]` annotation indicates that this macro should be made avaialable whenever the crate in which the macro is defined is brought into scope. The macro could not be brough into scope otherwise.
- The body is like a `match` expression if the code matches `($( $x:expr), *)` then the block of code following `=>` will be emitted.
	- `()` encompasses the whole pattern
	- `$()` captures values that match the pattern within the parentheses for use in the replacement code.
	- `$x:expr` matches any rust expression and gives the expression the name `$x`
	- `,*` following `$()` is the literal comma separater with a 0+ match wildcard for anything that precedes `*`.
	- In the body `$()*` is code that will be generated for each part that matches the `$()` in the pattern.
	- Calling `vec![1,2,3];` results in the following code being written.
		```
		{
    		let mut temp_vec = Vec::new();
			temp_vec.push(1);
			temp_vec.push(2);
			temp_vec.push(3);
			temp_vec
		}

		```

##### Procedural Macros for Generating Code from Attributes
- Accepts code as input, operates on that code, and produces some code as an output; rather than pattern matching and replacing like declarative macros.
- Definitions must reside in their own crate with a special crate type. This is for complex technical reasons.
- Can only be used on structs and enums.
- `some_attribute` is a placeholder for using a specific macro
```
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream { }

```
- Rust doesn’t have reflection capabilities, so it can’t look up the type’s name at runtime. We need a macro to generate code at compile time.
- `proc_macro` is the compilers API that allows us to read and manipulate Rust code from our code.
- `syn` crate parses Rust code from a string into a data structure that we can perfrom operations on.
- `quote` turns `syn` data structures back into Rust code.
- `stringify` takes literal code and turns it into output

##### Attribute-like Macros
- Similar to custom derive macros, but they allow you to create new attributes
- They are more flexible since they can be used on functions and other items.
- e.g.
```
#[route(GET, "/")]
fn index() {

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```
- In the above example: `attr` is for the `GET, "/"` and `item` is for what the attribute is attached to `fn index(){}`

##### Function-like Macros
- Defines macros that look like function calls.
- They can take an unknown number of arguments
- Can only be defined with the match like syntax
- Function-like macros take a `TokenStream` parameter and their definition manipultes that `TokenStrem` using Rust code as the other two types of procedural macros do.