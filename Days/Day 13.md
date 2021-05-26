## Day 13

#### 8.2 Storing UTF-8 Encoded Text with Strings
- Strings are a collection of bytes
- There is only one string type in the core of the language, a string slice `str` that is usually seen in its borrowed form `&str`
- Both `str` and `String` are UTF-8 encoded.
- The Rust standard library also has `OsString`, `OsStr`, `CString`, and `CStr`. With the latter of each being the borrowed variant.
  - `String` is valid UTF-8 that may contain zeros.
  - `OsString` is not constrained to adhere to Rust strings that are *valid UTF-8 and may contain zeros*
  - `CString` is a nul-terminated string with no nul bytes in the middle.
- created with `let mut s = String::new();`
- The `to_string` creates a `String`, the method is on any type that implements the `Display` trait; which string literals do.
- `String::from()` can also be used to create a `String`
- You can use the `+` operator or the `format!` macro to concatenate `String` values
- The `.push_str()` method is used to append a string slice.
- The `.push()` method appends a single character.
- Rust strings do not support indexing.
- A `String` is wrapped over a `Vec<u8>.`
- There are three ways to look at strings in Rust, as bytes, scalar values (characters), or grapheme clusters.
- You can create a string slice by referencing a range of bytes of a string.
  - e.g. `&mystring[0..4]`
  - If you try to create a string slice with bytes inbetween characters, Rust will panic at runtime as if an invalid index were accessed in a vector.
- You can iterate over a string with `.chars()` and `.bytes()`
  - Remember that valid unicode scalar values may be made up of more than one byte.

#### 8.3 Storing Keys with Associated Values in Hash Maps
- Has the signature  `HashMap<K,V>`
- create with the `HashMap::new()`
- You add new elements with `.insert()`
- Not included in the prelude so it must be explicityly brought into scope from collections
- You can also construct a hashmap by using iterators and the `collect` method on a vector of tuples.
  - e.g. `zip` combines two vectors into a vector of tuples and `collect` turns it into a `Hashmap`. It's possible to `collect` into multiple different data types so `HashMap<_,_>` is used. The underscores allow the compiler to infer the types.
  ```rust
  fn main() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
      teams.into_iter().zip(initial_scores.into_iter()).collect();
  }
  ```
- Types that have the `Copy` trait are copied into the hashmap, where owned values are moved and the hash map is the new owner. This means we won't be able to use the values after they have been *moved* into a hashmap with `insert`.
- If we enter references to values into the hash map, the values won't be moved. But the values that the references point to, must be valid for *at least* as long as the hash map is valid.
- Values can be retrieved with the `get` method. `get` returns an `Option<&V>`
- You can iterate with them over `for (key, value) in &myMap{}`
- *Overwrite*: By default `insert` will overwrite the previous key.
- *Insert if key has no value*: `.entry` takes a key you want to insert and gives you an `Entry` enum that represents a value that may or may not exist. `.or_insert` on `Entry` is defined to return a mutable reference to the value for the `.entry` key if it exists. Otherwise it enters the new value provided.
- *Update on*: Same as previous see, below example
```rust
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
    }

    println!("{:?}", map);
}
```
- The default hash function isn't the fastest as it is DoS resistant. You can replace it if needed by specifying a different *hasher* (a type that implmements `BuildHasher`). [crates.io](https://crates.io/) has libraries with common hashing algorithms.