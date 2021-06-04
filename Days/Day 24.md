#### 17.3 Implementing an Object-Oriented Design Pattern
- _/examples/blog_ implements a state pattern
- `take()` method calls on the option gives ownership to the caller and replaces the values with `None`
- Todo
	- [x]   Add a `reject` method that changes the post’s state from `PendingReview` back to `Draft`.
	- [x]   Require two calls to `approve` before the state can be changed to `Published`.
	- [x]   Allow users to add text content only when a post is in the `Draft` state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the `Post`.