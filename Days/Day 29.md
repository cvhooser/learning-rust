## Day 29
### `PartialEq`
- `PartialEq` lets you use the `=` and `!=` operators
- `PartialEq` on a struct means **all** fields have to be equal
- `PartialEq` On an enum means each variant is equal to itself

### `Eq`
- `Eq` trait can only be implemented if `PartialEq` is also implemented on a type. Although not all types that implement `PartialEq` can implement `Eq`
- `Eq` trait has no methods. It's purpose is to signal that every value of the annotated type is equal to itself. The object is the same (a reference would need to refer to the same object)

### `PartialOrd`
- `PartialOrd` lets you use the `<`, `>`, `<=`, and `>=` operators.
- `PartialOrd` trait can only be implemented on types with `PartialEq` traits
- Deriving `PartialOrd` implements the `partial_cmp` method which returns an `Option<Ordering>` that will be `None` when the values given don't produce an ordering.
- `PartialOrd` on enums and structs both return based on it's declaration order.

### `Ord`
- `Ord` trait allows you to know that for any two values of the annotated type, a vliad ordering will exist.
- `Ord` implements the `cmp` method and returns an `Ordering` because ordering will always be possible.
- `Ord` can only apply do types with `PartialOrd` and `Eq` (and `Eq` requires `PartialEq`).

### `Clone`
- `Clone` trait allows you to explicitly create a deep copy of a value, which could include copying heap data.
- Deriving `Clone` implements the `clone` method which calls `clone` on each of the parts of the type. This means all the fields or values in the type must also implement `Clone` to derive `Clone`

### `Copy`
- `Copy` trait allows you to duplicate a value by only copying bits stored on the stack and doesn't require any code.
- `Copy` trait doesn't define any methods.
- You can derive `Copy` on any type whose parts implement `Copy`
- A type that implements `Copy` must also implement `Clone` because a type that implements `Copy` has a trivial implementation of `Clone` that perfroms the same task as `Copy`
- Everything you can do with `Copy` you can accomplish with `Clone`. `Copy` is just an optimized version of it.

### `Hash`
- `Hash` trait allows you to take an instance of a type of arbitrary size and map that instance to a value for a fixed size. 
- Deriving `Hash` implements the `hash` method. The derived implementation combies the result of calling `hash` on each of the parts of the type, meaning all fields or values must also implement `Hash` to derive `Hash`.

### `Default`
- `Default` allows you to create a default value for a type.
- Deriving it implements the `default` function, which calls the `default` function on each part of the type. This means all part must implement `Default` in order to derive `Default`
- `Default::default` is commonly used in combination with the struct update syntax.
	- You can customize a few fields of a struct and then set and use a default value for the rest of the fields by using `..Default::default()`
- The `Default` trait is required when you use the method `unwrap_or_default` on `Option<T>` instances. `Option<T>` will return `Default::default` for the option `T` when the value is `None`.

### Useful Tools
- `rustfix` can automatically fix some compiler warnings. Also `cargo fix`
- Clippy: a collection of lints to analyze your code to catch common mistakes and improve your Rust code. `rustup component add clippy`
	- You can run clippy with `cargo clippy`
- To help IDE integration there is the *Rust Language Server* `rls`. This tool speaks the [Language Server Protocol](https://langserver.org/), a specification for IDE's and programming languages to communicate with each other.
- 