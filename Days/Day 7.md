## Day 7

### 3.0 Common Programming Concepts
- [Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
- [Operators](https://doc.rust-lang.org/book/appendix-02-operators.html)

#### 3.1 Variables and Mutability
- `const` must always be annotated with its type (use `const <var-name>: <type> = <value>;`) and is always immutable. i.e. cannot use `mut`.

#### 3.2 Data Types
- Integers (i32 is default)
  - Signed (negative/positive): i8, i16, i32, i64, i128, isize
  - Unsigned (positive): u8, u16, u32, u64, u128, usize
  - isize and usize refer to the architecture of the machine the code runs on (32 vs 64 bit)
  - isize and usize are primarily used for indexing a collection
- Integer Overflows
  - In debug mode the program will panic at runtime: `Unrecoverable Errors with panic!`
  - In release mode (using the `--release` flag), rust performs a **twos complement wrapping**.
    - Relying on this expected behavior is considered an error.
  - Solutions
    - Wrap in all modes with the `wrapping_*` methods
      - When wrapping is intended to happen e.g. modulus arithmetic
    - Return `None` if there is an overflow with the `checked_*` methods.
    - Return the value a boolean indicating whether there was overflow with the `overflowing_*` methods
    - Saturate at the values min or max values with `saturating_*` methods
      - Saturation stops values from being *greater than the max* or *less than the min*.
  - *Note* You can use `_` to separate longer integers for readibility. e.g. `1_000_000` = `1000000`
- Floating Points (f64 is default)
  - f32 and f64
    - f64 is roughly the same speed as f32 but with more precision
    - f32 is single precision, f64 is double precision
- Booleans
  - is either `true` or `false`.
  - uses `bool`, so `let f: bool = flase`
  - is one byte in size
- Charaters
  - signified by `''`
  - is four bytes
  - Represents a Unicode Scalar Value
- Compound Types
  - Tuples
    - tuples cannot grow or shrink once declared
    - uses `(, , ,)` syntax. e.g. `let tup: (i32, f64, u8) = (500, 6.4, 1);` or `let tup = (500, 6.4, 1);`
    - tuples can be destructured with pattern matching. I love destructuring with pattern matching!
    - tuples can be read by their index with `tup.[i]` e.g. `tup.0 == 500`
  - Arrays
    - all values must have the same type
    - arrays have a fixed length
    - uses `[, , , ]`
    - explicit syntax example `let a: [i32; 5] = [1, 2, 3, 4, 5];`
    - array is on the stack
    - If you don't know whether to use an array or a vector, use a vector
      - A vector is an array that can grow and shrink.
    - You can declare all the values a the start with `let a = [3; 5];`
      - This would be an array with all of length 5 all with a value of 3.
        - Can you use a generator in the first field?
    - access an element by `a[#]`
    - accessing an element outside the bounds of the array will cause a `panic!` i.e. runtime error.
      - The *panic* is due to a check at runtime that you are reading inside the bounds of the array. This means that you can not access invalid memory, unlike other languages.
      - e.g. *thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19 note: run with \`RUST_BACKTRACE=1\` environment variable to display a backtrace*