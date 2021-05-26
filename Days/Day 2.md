## Day 2
- [Debugger](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
	- e.g. `launch.json`
  ```json
      {
      "version": "0.2.0",
      "configurations": [
        {
          "type": "lldb",
          "request": "launch",
          "name": "Debug",
          "program": "${workspaceFolder}/<directory>/target/debug/<program-entry>",
          "args": [],
          "cwd": "${workspaceFolder}/<directory>/target/debug/",
          "sourceLanguages": ["rust"]
        }
      ]
    }

  ```
  - `unwap` is an error handler that uses `Result<T, E>` enum to return the value or the `panic!` the error.