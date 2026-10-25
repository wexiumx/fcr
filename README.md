# fcr - fast code runner

**Run any source file with one short command.**

```bash
fcr file.c
```

## What is it?

If you write low-level code (C, C++, Rust, ...), running a file is always two steps: compile first, then run. For C++ it looks like this:

```bash
g++ file.cpp -o file && ./file
```

That's a lot to type every single time. And if you juggle several languages, you also have to remember how each one is launched, so you end up googling the same commands again and again, just to run a program.

`fcr` fixes this. It figures out the language from the file and runs it for you. Same command, any language.

## Why do I need it?

- **One command for everything.** No more remembering `g++`, `rustc`, `zig run`, `go run` and friends.
- **Less typing.** `fcr file.c` instead of a long compile-and-run chain.
- **Run many files at once.** Pass several files (or a glob) and `fcr` runs them one after another.



## Installation

You need [Rust](https://rustup.rs/) installed.

```bash
cargo install --git https://github.com/wexiumx/fcr
```

Then try it:

```bash
fcr your_file.c
```



## Examples

Given this C++ file:

```cpp
#include <iostream>

int main() {
  std::cout << "Hello, from C++ file!\n";

  return 0;
}
```

Just run it:

```
❯ fcr cpp_file.c++
Hello, from C++ file!
```

Running many files at the same time:

```
❯ fcr ./*
Hello, from C file!

Hello, from C++ file!

Hello, from GO file!

Hello, from Rust file!

Hello, from ZIG file!
```

## Supported languages

- C
- C++
- Go
- Rust
- Zig
- Python
- Javascript
- Java
- C#
- R
- PHP
- Fortran
- Swift
- Haskell
- Elixir
- OCaml
- Common Lisp
- Scheme
- Cloujure
- Binary files





