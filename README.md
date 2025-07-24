# 0-shell

A simple, educational Unix-like shell written in Rust. It supports basic commands, custom parsing, colored output, and environment variable expansion.

---

## Features

- **Built-in commands:**  
  `ls`, `cd`, `pwd`, `cat`, `echo`, `cp`, `mv`, `rm`, `mkdir`, `clear`, `exit`
- **Colored prompt and output** (shows current directory and Git branch)
- **Custom shell parser:**  
  Handles quoting, escaping, tilde expansion, and environment variables
- **Signal handling:**  
  Graceful Ctrl+C support
- **Basic error reporting**
- **Brace expansion** (like `{a,b}` and `{1..5}`)
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

➜  echo "Double quoted $HOME"
Double quoted /home/youruser

➜  echo Hello\ World
Hello World
```

### Brace Expansion

```sh
➜  echo file{1..3}.txt
file1.txt file2.txt file3.txt

➜  echo {a,b,c}
a b c
```

### Home Directory Expansion

```sh
➜  cd ~
➜  pwd
/home/youruser
```

### Environment Variable Expansion

```sh
➜  echo $USER
youruser

➜  echo $PATH
/usr/local/bin:/usr/bin:/bin
```

### Signal Handling

Press `Ctrl+C` at any time to interrupt the current input and return to the prompt.

---
... (75 lines left)
Collapse
message.md
4 KB
use std::{
    env::{self},
    io::{self, Write},
};

pub fn parst_input(s: String, home_dir: &str) -> Result<Vec<String>, String> {
Expand
message.rs
11 KB
﻿
ysn_bhb
ysn_bhb
 
# 0-shell

A simple, educational Unix-like shell written in Rust. It supports basic commands, custom parsing, colored output, and environment variable expansion.

---

## Features

- **Built-in commands:**  
  `ls`, `cd`, `pwd`, `cat`, `echo`, `cp`, `mv`, `rm`, `mkdir`, `clear`, `exit`
- **Colored prompt and output** (shows current directory and Git branch)
- **Custom shell parser:**  
  Handles quoting, escaping, tilde expansion, and environment variables
- **Signal handling:**  
  Graceful Ctrl+C support
- **Basic error reporting**
- **Brace expansion** (like `{a,b}` and `{1..5}`)
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

➜  echo "Double quoted $HOME"
Double quoted /home/youruser

➜  echo Hello\ World
Hello World
```

### Brace Expansion

```sh
➜  echo file{1..3}.txt
file1.txt file2.txt file3.txt

➜  echo {a,b,c}
a b c
```

### Home Directory Expansion

```sh
➜  cd ~
➜  pwd
/home/youruser
```

### Environment Variable Expansion

```sh
➜  echo $USER
youruser

➜  echo $PATH
/usr/local/bin:/usr/bin:/bin
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

## More Examples

```sh
➜  mkdir testdir
➜  cd testdir
➜  echo "Hello" > hello.txt
➜  cat hello.txt
Hello

➜  cp hello.txt hello2.txt
➜  ls
hello.txt  hello2.txt

➜  rm hello2.txt
➜  ls
hello.txt

➜  cd ..
➜  rm -r testdir
```

---

## Author

#### yassine bahbib [https://github.com/ysnbhb/]
#### REDA ACHNIT  [https://github.com/achnitreda/]
#### Othman Qritel  [https://github.com/QOthman]