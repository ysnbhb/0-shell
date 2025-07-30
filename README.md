# 0-shell

A simple, educational Unix-like shell written in Rust. It supports basic commands, custom parsing, colored output, and environment variable expansion.

---

## Features

- **Built-in commands:**  
  `ls`, `cd`, `pwd`, `cat`, `echo`, `cp`, `mv`, `rm`, `mkdir`, `clear`, `exit`
- **Colored prompt and output** (shows current directory and Git branch)
- **Custom shell parser:**  
  Handles quoting, escaping, tilde expansion
- **Signal handling:**  
  Graceful Ctrl+C support
- **Basic error reporting**
- **Home directory expansion** (`~`)

---

## Getting Started

### Prerequisites

- Rust (https://rustup.rs)
- Rust version 1.88+

### Build and Run

```sh
git clone https://github.com/ysnbhb/0-shell.git
cd 0-shell
cargo run  or make
```

---

## Usage Examples

### Basic Commands

```sh
➜  ls
➜  cd src
➜  pwd
➜  cat main.rs
➜  echo "Hello, world!"
➜  exit
```

### Quoting and Escaping

```sh
➜  echo "This is a string with spaces"
This is a string with spaces

➜  echo 'Single quoted $HOME'
Single quoted $HOME

➜  echo Hello\ World
Hello World
```

### Home Directory Expansion

```sh
➜  cd ~
➜  pwd
/home/youruser
```

### Signal Handling

Press `Ctrl+C` at any time to interrupt the current input and return to the prompt.

---

## Project Structure

```
0-shell/
├── src/
│   ├── main.rs              # Entry point: shell loop and signal handling
│   ├── shell.rs             # Core shell logic: REPL, parsing, dispatch
│   ├── commands/
|   |   |__ls/  # `ls` implementation
|   |   |   |__commend.rs
|   |   |   |__handle_flag.rs
|   |   |   |__mod.rs       # Command registry & dispatcher
|   |   |   |__permission.rs
|   |   |   |__print_ls.rs
|   |   |   |__struct.rs
│   │   ├── mod.rs           # Command registry & dispatcher
│   │   ├── cd.rs            # `cd` implementation
│   │   ├── echo.rs          # `echo` implementation
│   │   ├── pwd.rs           # `pwd` implementation
│   │   ├── cat.rs           # `cat` implementation
│   │   ├── cp.rs            # `cp` implementation
│   │   ├── rm.rs            # `rm` implementation
│   │   ├── mv.rs            # `mv` implementation
│   │   ├── mkdir.rs         # `mkdir` implementation
│   │   └── exit.rs          # `exit` implementation
│   ├── utils/
│   │   ├── io.rs            # Input/output helpers
│   │   ├── fs.rs            # Filesystem helpers
│   │   └── parser.rs        # Argument/token parsing
│   │   └── mod.rs            # Command registry & dispatcher
│   │   └── error.rs
│   │   └── color.rs
│   └── signals.rs           # Ctrl+C signal handlers
├── Cargo.toml
└── README.md
```

---

## Contributing

Pull requests and issues are welcome!  
Feel free to fork and experiment.

---

## Author

#### [yassine bahbib](https://github.com/ysnbhb/)
#### [REDA ACHNIT](https://github.com/achnitreda/)
#### [Othman Qritel](https://github.com/QOthman/)
#### [Abdessamad Mazighi](https://github.com/amazighii/)

________________
