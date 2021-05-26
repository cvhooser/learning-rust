## Day 22

#### 15.5 `RefCell<T>` and the Interior Mutability Pattern
- This pattern uses an `unsafe` code block inside a data structure to bend Rust's rules around mutation and borrowing.
- Single ownership
- Borrowing rules' invariants are forced at runtime instead of compile time
- Instead of failing to compile `RefCell<T>` will panic and exit if you break the rules
- The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, whereas they are disallowed by the compile-time checks.
- Givens the illusion of immutability to the outside world, but can mutate itself.
- Refer to project *./examples/tracking*
- `borrow` and `borrow_mut` return smart pointers `Ref<T>` and  `RefMut<T>` respectively.
- If you try to `borrow_mut` more than once the program will panic and exit
- Keeping track of borrows has a slight runtime penalty
- Combing `Rc<T>` and `RefCell<T>` can let you have multiple owners of mutable data.


#### 15.5 Reference Cycles Can Leak Memory
- Memory leaks (while difficult to make happen) are memory safe in rust
- It is possible to create references where items refer to each other and the values will never be dropped
- It is hard to create reference cycles, but when using `RefCell<T>` and `Rc<T>` in nested combinations you should be careful not to create one. Since this is a condition in which they can occur.
- A way to structure the data is that cycles have some owner relationships and some non-owner relationships.
	- You can add these non-owner relationships with `Rc::downgrade(var: Rc<T>)`. This increases the `weak_count` of an `Rc<T>` by 1.
	- The `weak_count` does not have to be 0 for the `Rc<T>` instance to be cleaned up, just the `strong_count`
- Before using any value from a `Weak<T>` you have to first make sure that the reference exists and hasn't been dropped.
	- This is done by calling `Rc::upgrade(var: Weak<T>` which will return an `Option<Rc<T>>`


### 16.0 Fearless Concurrency
- Ownership and type systems are also good tools for solving concurrency, as it makes race conditions compile time issues.

#### 16.1 Using Threads to Run Code Simultaneously
- Languages that call the OS API to create new threads is called a *1:1*; one operating thread per language thread.
- Programming language provided threads are known as *green threads*
	- This is known as an *M:N* model. There are *M* green threads per *N* operating system threads.
- The trade-off that is most imporant to Rust is runtime support. By *runtime* we mean code that is included by the language in every binary. Languages that have *no runtime* usually have very small binaries.
- Rust's thread implementation is a *1:1* as to keep the language runtime small. However there are crates that implement *M:N* threading.
- We can create new threads with `thread::spawn`
- You can call `join` on a `JoinHandle` (the return value from `thread::spawn`) to have the calling thread wait for `join`ed thread to finish. i.e. it blocks the calling thread so the called thread can finish execution
- `move` closure allows you to use data from one thread in another thread.
	- `move` just needs to be declared before the closure and Rust will move ownership of the variables used in the thread

#### 16.2 Using Message Passing to Transfer Data Between Threads
- One increasingly popular approach to ensuring safe concurrency is _message passing_, where threads or actors communicate by sending each other messages containing data. Here’s the idea in a slogan from [the Go language documentation](https://golang.org/doc/effective_go.html#concurrency): “Do not communicate by sharing memory; instead, share memory by communicating.”
- Rust's message passing solution is to use a *channel*
	- A channel has two halves: a *transmitter* and a *receiver*
	- A channel is *closed* if **either** the *transmitter* or *receiver* half is dropped
	- You can create a channel with `mpsc::channel`, which stands for *multiple producer, single consumer* that returns a tuple with *transmitter* and *receiver*
	- `recv` will block until a value is sent or the channel closes
	- `try_recv` doesn't block and returns immediately if there is a value and `Err` if there isn't any at this time.
- The `send` function passes ownership to the receiving thread.
- We can create a second producer by calling `clone` on the first one.

#### 16.3 Shared-State Concurrency
- *mutex* is short for *mutal exclusion*. It only allows a single thread to access data at a time.
- Mutexes are notorious for being hard to use because:
	1. You must attemp to acquire the lock before using the data
	2. When you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
- `lock` acquires the lock and blocks the current thread until that lock is acquired. The call would fail if another thread holding the lock panicked and no one would ever be able to get the lock. Using `unwrap` causes this thread to panic in that situation.
	- Once the `lock` is acquired we can treat it as a mutable reference (which you can rightfully assume means `Mutex` is a smart pointer)