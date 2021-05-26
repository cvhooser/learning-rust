## Day 1
- Tools
	- [VSCODE](https://code.visualstudio.com/) with [Rust Extension Pack](https://marketplace.visualstudio.com/items?itemName=swellaby.rust-pack)
	- [Getting Started](https://www.rust-lang.org/learn/get-started)

- Build Tools
	- Cargo is a build tool that helps manage dependencies known as `crates`
	- Project information and dependencies exist in `Cargo.toml`

- Compiling
	- `rustc` is used to compile the code
	- `cargo build` installs dependencies
		- compiles into `./target/debug/<project-name>`
		- creates a `cargo.lock` file for dependency tracking
	- `cargo run` compiles and runs the program
	- `cargo check` will tell us if you program compiles or not
	- `cargo build --release` will build with optimizations for releases

- Formatting
	- For single files you can use `rustfmt <file>.rs`
		- The default is 4 spaces (I hate that) so I adjusted it to 2 for sanity sake, besides that I left everything the same so I can learn idomatic Rust
	- For `cargo` projects you can run `cargo fmt` to format the entire project
	- You can add a `rustfmt.toml` file to the project directory that will be used with both format commands.
	- Add a `--check` flag to see the changes (if any) that would be applied when you change

- Syntax
	- `<command>!` refers to a macro, NOT a function call