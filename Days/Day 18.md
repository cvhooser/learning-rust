## Day 18

### 12.0 An I/O Project: Building a Command Line Program
- So we took a bit of time off. So far the important bits of this project are about binary project separation.
	- Split your program into a _main.rs_ and a _lib.rs_ and move your program’s logic to _lib.rs_.
	- As long as your command line parsing logic is small, it can remain in _main.rs_.
	- When the command line parsing logic starts getting complicated, extract it from _main.rs_ and move it to _lib.rs_.
- There’s a tendency among many Rustaceans to avoid using `clone` to fix ownership problems because of its runtime cost
- `dyn` is short for *dynamic*
-  It was discussed earlier, but `?` returns error (i.e. error propogation)
-  Returning `Ok(())` is the idiomatic way to know you are calling a function for its side effects only.
-  `if let` if we only have an error case
-  `unwrap` if we want to `panic!`
-  `unwrap_or_else` if we want to handle an error case and have `Ok(v)` that we need to unwrap