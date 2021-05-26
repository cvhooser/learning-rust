## Day 9
#### 4.2 References and Borrowing
- `&` lets you create a reference that does not own a variable.
- Having references as function parameters is known as borrowing.
- You CANNOT change something you are borrowing.
- Mutable References
  - You can change it if you make it a mutable reference
  - You can only have ONE mutable reference to a particular piece of data in a particular scope.
    - This prevents a data race from occuring
      * Two or more pointes access the same data at the same time.
      * At least one of the pointers is being used to write to the data.
      * There's no mechanism being used to synchronize access to the data.
    - You cannot mix mutable and immutable references
      - e.g. 
      ```rust
          let mut s = String::from("hello");
          let r1 = &s; // no problem
          let r2 = &s; // no problem
          let r3 = &mut s; // BIG PROBLEM
          println!("{}, {}, and {}", r1, r2, r3);
      ```
    - *Note* A references scope starts where it is introduced and ends on its last usage.
      - That is why this example works:
      ```rust
          let mut s = String::from("hello");
          let r1 = &s; // no problem
          let r2 = &s; // no problem
          println!("{} and {}", r1, r2);
          // r1 and r2 are no longer used after this point
          let r3 = &mut s; // no problem
          println!("{}", r3);
      ```
- Dangling References
  - Having reference to a memory for a variable that had previously gone out of scope.
    - Of course Rust protects against this
- Overview
  - At any given time you can have either one mutable reference or multiple immutable references
  - References will always be valid

#### 4.3 The Slice Type
- Lets you reference a contiguous sequence of elements in a collection.
- String Slices
  ```rust
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
  ```
  - Leading and trailing numbers aren't necessary for the beginning and the end.
    e.g. `&s[..2]` and `&s[3..]`
  - *Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.*
  - the function signature for string slices is `fn first_word(s: &str) -> &str { }`
  - String literals are also string slices
- Other Slices
  - There is a general array slice with the same syntax as a string slice.