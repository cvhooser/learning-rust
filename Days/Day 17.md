## Day 17

### 11.0 Writing Automated Tests

#### 11.1 How to Write Tests
- Bodies of test functions have three parts
  1. Set up any needed data or state.
  2. Run the code you want to test.
  3. Assert the results are what you expect.
  - You can do these with the `test` attribute, a few macros, and the `should_panic` attribute.
- test functions are marked with the `[#test]` attribute
- `cargo test` runs all tests in our project
- tests run fall into: *passed*, *failed*, *ignored*, *measured*, or *filtered*.
  - *Measured* tests are for benchmarks that measure performance
- `Doc-tests` ouput is for the result of documentation tests. Rust can compile code examples that appear in API documentation.
- Each test is run in a new thread and when the main thread sees that a test thread has died, the test is marked as failed.
- The `assert` marco ensures that some condition evaluates to `true`.
- You can compare to a value with `assert_eq!(v1, v2)` and `assert_ne!(v1, v2)`
- Instead of *expected* and *actual*, it is *left* and *right* where order doesn't matter.
  - Arguments must implement the `PartialEq` and `Debug` traits. This is usually as simple as adding `#[derive(PartialEq, Debug)]` since they are derivable traits.
- Adding custom failure messages
- The second parameter is a custom error message that uses `format!` on the following parameters.
- You can check that code will panic, by adding the `#[should_panic]` trait to a test.
- To make `#[should_panic]` more precise, we can give it an `expected` value. `expected` must be a substring of the panic message to pass.
- You can write tests with `Result<T, E>`
  - Writing tests with it can allow you to use `?`. Which is a convenient way to write tests that should fail.
  - You CANNOT use the `#[should_panic]` annotation with them. 

#### 11.2 Controlling How Tests Are Run
- `cargo test` creates a binary and runs it. You can use commands with test and on the binary. `--` separates where the command is directed.
  - e.g. `cargo test --help` vs `cargo test -- --help`
- Tests run in parallel by default. You can restrict this by using the `--test-threads` flag
  - e.g. `cargo test -- --test-threads=1`
- We get output that is printed in failed tests only. We can change this with the `--show-output` flag.
  - e.g. `cargo test -- --show-output`
- We can pass the name of any test function to `cargo test` to run only that function. If we had more tests that didn't run they would show up under *filtered*
  - e.g. `cargo test my_function`
- You can pass part of a test name and cargo will run all tests that contain that phrase.
  - e.g. `cargo test my`
- We can also run all tests in a module the same way because the module becomes part of the test names `test tests::greeting_contains_name`
- `#[ignore]` allows you to ignore tests
  - We can run only these tests with `cargo test -- --ignored`

#### 11.3 Test Organization
- Two main types: *unit tests* and *integration tests*
- Unit tests 
  - Go in the *src* directory. They go in the file with the code they are testing. The convention is to have a module named `tests` in each file to contain the test functions and to annotate the module with `#[cfg(test)]`
  - The `#[cfg(test)]` annotation tells rust to only compile and run the test code when you run `cargo test`
  - `cfg` stands for configuration
  - Rust allows testing of private functions since testing is done in the same file.
- Integration Tests
  - Entirely external to your library
  - exist in a `tests` directory and only get compiled when we run `cargo test`
  - Each file in the *tests* directory is compiled as its own separate crate.
  - The output from `cargo test` is *unit tests*, *integration tests*, *doc tests*.
  - Each integration test file has its own section
  - We can limit the integration tests we run by using the `--test` argument.
    - e.g. `cargo test --test integration_test`
  - If you want to have shared functions for setup, teardown, etc. They need to follow the naming convention `tests/common/mod.rs` where *common* can be anything. Subdirectories of *tests* do not get compiled as separate crates or have sections in the output.
  - We would add the code to our test file with `mod common`.
- If we only have a binary crate that contains a *src/main.rs* and doesn't have a *src/lib.rs* we can't create integration tests. **Only library crates can expose functions for other crates to use; binary crates are mean't to be run on their own.**
- This is one of the reasons Rust projects with a binary have a straightforward *src/main.rs* that calls logic that lives in *src/lib.rs* Integration tests can test the library crate and assume the small amount of code in *src/main.rs* will be fine.