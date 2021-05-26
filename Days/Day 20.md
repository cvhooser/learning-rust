 ## Day 20
 
 ### 14.0 More about Cargo and Crates.io
 - Customize you build through release profiles
 - Publish libraries on [crates.io](https://crates.io/)
 - Organize large projects with workspaces
 - Install binaries from [crates.io](https://crates.io/)
 - Extend Cargo using custom commands

#### 14.1 Customizing Builds with Release Profiles
- Cargo has two main profiles *dev* and *release*
- `[profile.*` in the Cargo.toml will override defaults
- `opt-level` controls the number of optimizations Rust will apply

#### 14.2 Publishing a Crate to Crates.io
- `///` is documentation comment that will generate html.
	- Mean't for public api about how to use your project
- `cargo doc` creates documentation in *target/doc* (by running the **rustdoc** tool) as well as documentation for all.
	- Running `cargo doc --open` will build then open docs in the web browser.

**Commonly Used Sections**
	- *Panics*: scenarios in which the functions panics
	- *Errors*: describing the kinds of errors if the function returns a `Result`
	- *Safety*: explain why there is an `unsafe` block if there is one

**Document Comments as Tests**
	- `cargo test` runs code examples in your documentation as tests

**Commenting Contained Items**
	- `//!` adds documents to the item that contains the comments rather than adding documentation to the items following the comments.
		- These are more high level comments
		
**Exporting a Convenient Public API with `pub use`
- `pub use` can re-export items to make a public structure that is different than your private structure.
	- e.g. in `src/lib.rs` put `pub use self::<some>::<thing>;`

**Setting Up a Crates.io Account**
- After an account is created, get your api key and run  `cargo login <api-key`
	- Gets stored locally in *~/.cargo/credentials*
- Crates must have a unique name on [crates.io](https://crates.io/)
- `cargo publish` will publish the crate to [crates.io](https://crates.io/)
- You must have a *name*, *description*, *author*, and *license* where the *license* is a license identier value that is listed here: [Linux Foundation’s Software Package Data Exchange (SPDX)](http://spdx.org/licenses/).
	- *If you want to use a license that doesn’t appear in the SPDX, you need to place the text of that license in a file, include the file in your project, and then use `license-file` to specify the name of that file instead of using the `license` key.*
	- e.g. `license = "MIT"`
	- To use a license that doesn't appear in the SPDX, you need to place the text of that license in a file, include the file in your project, and then use `license-file` to specify the name of that file instead of using the license key.
- Publish is permanent
	- The version can never be overwritten and **the code cannot be deleted**
- To publish a new version:
	- Use the [Semantic Versioning rules](http://semver.org/) to decide what an appropriate next version number is based on the kinds of changes you’ve made
	- Run `cargo publish` to upload the new version.

**Removing Versions from Crates.io with `cargo yank`**
- Yanking prevents new projects from starting to depend on that version while allowing existing projects that depend on it to continue to download and depend on that version.
	- i.e. Cannot create a *Cargo.lock* but existing *Cargo.lock* files will work.
	- e.g. `cargo yank --vers 1.0.1`
- You can undo a yank with the `--undo` flage
	- e.g. `cargo yank --vers 1.0.1 --undo`
- **Yank does not delete any code**

#### 14.3 Cargo Workspaces
- A workspace is a set of packages that share the same *Cargo.lock* and output directory.
- A way to organize overly large packages.
- The workspace only has one *target* directory. Even subdirectors end up in the base directory.
- Add workspaces as dependencies in the parent toml. They can then be included with `use <project>`
- We can run a binary crate from the parent directory by selecting which package with the `-p` flag.
	- e.g. `cargo run -p adder`
- All workspaces have one Cargo.lock ensuring they share all dependencies. However packages have to be added independently to each workspace.
- `cargo test` run all tests in all workspaces, you can run specific workspaces with the `p` flag
	- e.g. `cargo test -p add-one`
- **You must publish each crate in it's workspace directory**. There is no way to publish from the root of all workspaces

#### 14.4 Installing Binaries from Crates.io with Cargo install
- `cargo install` allows you to install and use binary crates locally.
- convienent way to install tools from [crates.io](https://crates.io/)
- All installed crates are stored in the root's *bin* folder.
	- Default directory is `$HOME/.cargo/bin`


#### 14.5 Extending Cargo with Custom Commands
- Cargo is designed so you can extend it with new subcommands
- If a binary in your `$PATH` is named `cargo-<something>`, you can run it as if it was a subcommand, i.e. `cargo <something>`.
- This means you can use `cargo install` to install extensions and then run them just like the built-in Cargo tools