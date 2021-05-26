## Day 21

### 15.0 Smart Pointers
- Smart pointers are data structures that act as a pointer with additional metadata and capabilities
	- `String` and `Vec<T>` are both smart pointers
- Smart pointers implement `Deref` and `Drop` traits.
	- `Deref` allows an instance of the smart pointer to behave like a reference so you can write code that works with either references or smart pointers.
	- `Drop` allows you to customize code that is run when an instance of the smart pointer goes out of scope.
- Commonly used smart pointers are:
	- `Box<T>` for allocating values on the heap
	- `Rc<T>` that is a reference counting type that enables multiple ownership
	- `Ref<T>` and `RefMut<T>` that are accessed through `RefCell<T>`. A types that enforces the borrowing rules at runtime instead of compile time.
- *Interior Mutability* is a pattern where an immutable type exposes an API for mutating an interior values.

#### 15.1 Using `Box<T>` to Point to Data on the Heap
- No overhead other than storing data on the heap instead of the stack
- Situations to use `Box<T>`
	1.  When you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size
	2. When you have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so.
	3. When you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type.
- Case 2. allows us to store the data on the heap and only copy over the pointer data on the stack when we change ownership (as opposed to copying over all of the data)
- Case 3. is known as a *trait object*.

**Enabling Recursive Types with Boxes**
- At compile time Rust needs to know how much space a type takes up. The size of a *recursive type* cannot be known at compile time. Wrapping it in a Box allows it to have a known size.

**Cons List**
- A data structure that comes from Lisp
- Constructs a new pair from two arguments (usually a single value and another pair). These pairs containing pairs form a list.
- `cons` is short for "construct function"
- Each item in a cons list contains two elements: the value of the current item and the next item. The last item contains a value called `Nil` without a next item.
- A cons list is produced by recursively calling the `cons` function.
- e.g. This throws an error "recursive type 'List' has infinite size"
```rust
enum List { 
  Cons(i32, List), 
  Nil, 
}


fn main() {
  let list = Cons(1, Cons(2, Cons(3, Nil))); 
}
```


**Using `Box<T>` to Get a Recursive Type with a Known Size**
- Wrap list in `Box<T>`
- e.g.
```rust
enum List { 
  Cons(i32, Box<List>), 
  Nil, 
}


fn main() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); 
}
```
- This works because the compiler now knows it only needs space for an `i32` and for the pointer.

#### 15.2 Treating Smart Pointers Like Regular References with the Deref Trait
- implemnting `Deref` allows you to customize the behavior of the *deference operator* i.e. `*` 
- `*` operator works the same way for `&var_name` and `Box::new(var_name)`
- `*` operator is replaced with `*(var.deref())`. This is done so that the ownership isn't transferred to the scope that used the deference operator.

**Implicit Deref Coercions with Functions and Methods**
- *Deref coercion* is a convenience that Rust performs on arguments to functions and methods.
- When the `Deref` trait is defined for the types involved, Rust will analyze the types and use `Deref::deref` as many times as necessary to get a reference to match the parameter’s type. The number of times that `Deref::deref` needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!

**How Deref Coercion Interacts with Mutability**
- You can use `DerefMut` to override the * operator on mutable references*
- Three cases where deref coercion happens
	1. From `&T` to `&U` when `T: Deref<Target=U>`
	2. From `&mut T` to `&mut U` when `T: DerefMut<Target=U`
	3. From `&mut T` to `&U` when `T: Deref<Target=U>`
- It is not possible to coerce an immutable reference to a mutable one. This is because it can possibly break the borrowing rules of only having one mutable reference.

#### 15.3 Running Code on Cleanup with the Drop Trait
- `Drop` can be implemented on any type and is almost always used when implemented a smart pointer.
- Calling `drop` explicitly is not allowed by default because it would create a *double free*. That is Rust will call it at the end of the scope after it was called earlier in the scope.
- You can drop a value early with `std::mem::drop`
	- example use case would be to force `drop` when the underlying smart pointer is managing locks. That way you could release the lock for it to be acquired by something else.
	- It is in the prelude so it does not need to be brought into scope.

#### 15.4 `Rc<T>`, the Reference Counter Smart Pointer
- Used when we have multiple parts of our program that will read data on the heap, but we are unsure who will be the last to read it.
- Only for use in Singled Threaded scenarios
- `Rc::clone(&var)` will increase the reference count and the data won't be cleaned up unless there are zero references to it.
	- We could have also used `var.clone()` but convention is to follow the above.
	- The reason is that we can easily distinguish between deep-copy clones and reference count clones that otherwise could not be obvious.
- ` Rc::strong_count(&var)` will tell you how many references there are for a piece of data.
	- This is called `strong_count` because there is also a `weak_count` that is used to prevent reference cycles.
- The implementation of `Drop` automatically decreases the count.