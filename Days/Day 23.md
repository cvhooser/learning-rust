#### 16.3 Shared-State Concurrency
- *mutex* is short for _mutal exclusion_. It only allows a single thread to access data at a time.
- Mutexes are notorious for being hard to use because:
	1. You must attemp to acquire the lock before using the data
	2. When you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
- `lock` acquires the lock of `Mutex` and blocks the current thread until that lock is acquired. The call would fail if another thread holding the lock panicked and no one would ever be able to get the lock. Using `unwrap` causes this thread to panic in that situation.
	- Once the `lock` is acquired we can treat it as a mutable reference (which you can rightfully assume means `Mutex` is a smart pointer)
- `lock` returns a smart pointer called `MutexGuard` that is wrapped in a `LockResult`.
- Locks are automatically released when the `MutexGuard` goes out of scope.
- Use `Arc<T>` for thread safe sharing of data. `Arc` stands for *atomic reference counted*
- The `Mutex<T>/Arc<T>` is the  `RefCell<T>/Rc<T>` combination in a concurrent environment.
	- `Mutex<T>` does come with the risk of _deadlocks_. This can occur when two locks are needed and two threads each have one of the locks.

#### 16.4 Extensible Concurrency with the Sync and Send Traits
- Rust has two concurrency concepts embedded in the language `std::marker` and Traits `Sync` and `Send`.
- `Send` trait allows ownership transferrence between threads. Almost everything in Rust implements `Send`.
	- Any type composed entirely of `Send` types is automatically marked `Send` as well.
- `Sync` trait indicates that the type implementation can be referenced from multiple threads. In other words, any type `T` is `Sync` if `&T` (a reference to `T`) is `Send`, meaning the reference can be sent safely to another thread.
- **Implementing Send and Sync Manually is Unsafe**
	- They are marker traits (have no methods to implement) and automatically get applied based on composite types.
	- They are useful for enforcing invariants related to concurrency.
	- Manually implementing these ivolves implementing `unsafe` Rust code.
- Using `Mutex<T>` and `Arc<T>` ensure race conditions don't happen.

### 17.0 Object Oriented Programming Features of Rust
- Discuss what parts of Rust are OO and what parts are not.

#### 17.1 Characteristics of Object Oriented Languages
- `struct`'s with `impl` blocks are effectively objects.
- `pub` keyword is a way to to _encapsulation_ and obscure implementation details to offer and structured api for consumers to use.
- Rust _DOES NOT_ have inheritance
	- It does have _default_ implementations on Traits that can be _overridden_ by types implementing those traits
- Rust trait bounded syntax is a type of _polymorphism_ called _bounded parametric polymorphism_.

#### 17.2 Using Trait Objects That Allow for Values of Different Types
- Trait objects cannot have data
- When defining that a type should be a _Trait Object_ we need to use `Box<T>` to ensure the data is on the heap and then `dyn T` which stands for _Dynamically Sized Types and the `Sized` Trait_.
	- This works differently from defining a struct that uses a generic type parameter with trait bounds. A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
	- This concept—of being concerned only with the messages a value responds to rather than the value’s concrete type—is similar to the concept of _duck typing_ in dynamically typed languages: if it walks like a duck and quacks like a duck, then it must be a duck!
- The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn’t implement a method but we call it anyway. Rust won’t compile our code if the values don’t implement the traits that the trait objects need.
- _Trait Objects_ use _dynamic dispatch_ instead of _static dispatch_ that is used by Generics.
	- Because the Rust compiler does not know which method to call on which type at compile time, Rust uses the pointers inside the trait object to know which method to call.
	- This also means the compiler cannot inline the code and therefore cannot optimize to some extent. Basically we are trading optimization for flexibility.
- Traits that we want to use as Trait Objects must be _object-safe_. That is:
	1. The return type isn't `Self`
	2. There are no generic type parameters.
	- Trying to use `Self` will put us in a pardox because the the compiler no longer knowns the concrete type, but is being told to use the concrete type as a return. 
		- The below would give us an error.
	```
	pub struct Screen { 
		pub components: Vec<Box<dyn Clone>>, 
	}
	```
- More details about object safety can be found [here](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md)
