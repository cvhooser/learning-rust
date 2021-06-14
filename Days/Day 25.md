## Day 25

### 18.0 Patterns and Matching
- `match` keyword can match against a pattern of the following types
	- Literals
	- Destructured arrays, enums, structs, or tuples
	- Variables
	- Wildcards
	- Placeholders

#### 18.1 All the Places Patterns Can Be Used
- `match` expressions have multiple arms
```
	match VALUE {
		PATTERN => EXPRESSION,
		PATTERN => EXPRESSION,
		PATTERN => EXPRESSION
	}
```
- match expressions must be exhaustive
	- A common way to do this is with a catch all pattern as the last arm i.e. `_`
- `if let` can contain an `else` expression
-  It is also possible to use `if let`, `else if`, and `else if let` together
-  The downside of using `if let` expressions is that the compiler doesn’t check exhaustiveness, whereas with `match` expressions it does.
-  `while let` will loop as long as the pattern continues to match
-  In a `for` loop the pattern is the value that directly follows `for`
	-  e.g.
	```
	let v = vec!['a', 'b', 'c']; 
	
	for (index, value) in v.iter().enumerate() { 
		println!("{} is at index {}", value, index); 
	}
	```
- The basic `let` expression is pattern match. So `let x = 5` is actually `let PATTERN = EXPRESSION;`.
	- e.g.  `let (x, y, z) = (1, 2, 3);`
- Function parameters can also be patterns.
	- e.g.
	```
	fn print_coordinates(&(x, y): &(i32, i32)) { 
		println!("Current location: ({}, {})", x, y); 
	} 
	
	fn main() { 
		let point = (3, 5); 
		print_coordinates(&point); 
	}
	```
- You can use pattern matching in closure parameter lists like you can with function arguments.

#### 18.2 Refutability: Whether a Pattern Might Fail to Match
- Patterns are either _refutable_, can fail to match or _irrefutable_, cannot fail to match.
- Function parameters, `let` statements, and `for` loops can only accept _irrefutable_ patterns.
- `if let` and `while let` expressions accept _refutable_ and _irrefutable_ patterns (but the compiler will complain against _irrefutable_ patterns)
- `match` arms must use _refutable_ patterns

#### 18.3 Pattern Syntax
- You can match multiple patterns using the `|` syntax which means _or_.
- You can match to an inclusive range with `..=`. Which means that any value inside of the range, inclusive of bounds will match.
	- Ranges are only allowed with numeric values and `char` values.
		- e.g. `1..=5` or `'a'..='j'`
- You can destructure structs with special syntax
	- e.g.
	```
	struct Point {
		x: i32,
		y: i32
	}
	
	fn main() {
		let p = Point {x: 0, y: 7};
		
		// If you want the destructure variables to have different names
		//let Point { x: a, y: b } = p;
		let Point {x, y} = p;
		assert_eq!(0, x);
		assert_eq!(7, y);
	}
	```
- We can use matching to separate different cases of a struct
	- e.g. Points on x-axis, points on y-axis, and neither axis.
	```
	struct Point {
		x: i32,
		y: i32
	}
	
	fn main() {
		let p = Point {x: 0, y: 7};
		
		match p {
			Point { x, y: 0 } => println!("On the x axis at {}", x),
			Point { x: 0, y } => println!("On the y axis at {}", y),
			Point { x, y } => println!("On neither axis: ({}, {})", x, y),
		}
	}
	
	```
- We can use destructuing for `enums`, like we do with `Option<T>`, but it has to match the way the data is stored within the enum
- e.g.
```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}
```
- Destructing can be used on nested enums and structs
- We can use `_` and `..` to ignore values we don't care about in a match expression
	-  `_` does not bind the variable
	-  e.g. ignoring values in a struct with `..`
	```
	fn main() {
		struct Point {
			x: i32,
			y: i32,
			z: i32,
		}

		let origin = Point { x: 0, y: 0, z: 0 };

		match origin {
			Point { x, .. } => println!("x is {}", x),
		}
		
		// Same as above
		/*
		match origin {
			Point { x, y: _, z: _ } => println!("x is {}", x),
		}
		*/
	}
	```
	- using `..` must be unambiguous or Rust will throw a compiler time error.
		- e.g. Using it to get the second value of a tuple
- A _match guard_ is a conditional `if` condition specified after the pattern in a `match
	- e.g.
	```
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
	```
- You can use the `|` operator with a  _match guard_ to run the the _match guard_ on all of the patterns.
	- e.g.
	```
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
	```
- We can use the `@` operator to assign a variable that holds a value at the same time we are testing it. `@` allows us to capture whatever value matched a given pattern.
	- e.g.
	```
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
		// We don't know what the id is here as opposed to the above
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
	```
