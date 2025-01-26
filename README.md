## Devil: A Tool for Project Management

A command-line tool to simplify project management. Devil offers functionalities to:

* Create new projects with ease

* View project directory structure
## Installation

Devil is a Rust program. To install it, follow these steps:

1. Clone this repository:

```bash
git clone https://github.com/BlueMoonHunt/devil.git
```

2. Navigate to project directory

```bash
cd devil
```

3. Build project in release mode
```bash
cargo build --release
```

4. Add the executable to your PATH environment variable.The executable will be located at 
```target/release/devil.```

## Include Devil in your PATH

### Linux/macOS

1. Open your shell configuration file (e.g., .bashrc or .zshrc) using a text editor.

2. Add the following line to the file, replacing ```/path/to/devil``` with the actual path to the directory containing the ```devil``` executable.

```bash
export PATH="$PATH:/path/to/devil/target/release"
```
3. Save the changes to your shell configuration file.

4. Source the file to apply the changes immediately:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

### Windows

1. Search for "Environment Variables" in the Start Menu and select "Edit the system environment variables".
2. Click on "Environment Variables...".
3. Under "System variables", find the "Path" variable and click "Edit...".
4. Click "New" and add the path to the directory containing the ```devil.exe``` executable
5. Click "OK" on all dialogs to save the changes.
## Project Languages Implemented

* C/C++ with cmake
* Rust

## Usage/Examples

Creating a new Project

The new project is created in `~/Dev/` : default

```bash
devil project my_project Rust
```

Show Status

```bash
devil status path/to/project --ignore .git target
```

```bash
├── .gitignore  # File to specify files to exclude from Git version control
├── Cargo.lock   # (For Rust projects) Dependency lock file
├── Cargo.toml   # (For Rust projects) Project configuration file
└── src           # (For Rust projects) Source code directory
    └── main.rs     # (For Rust projects) Main source code file
```

## License

[MIT](https://choosealicense.com/licenses/mit/)

