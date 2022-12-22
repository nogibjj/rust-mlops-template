


# rust-mlops-template
A work in progress to build out solutions in Rust for MLOPs

## Motivation

One of the key goals of this project is to determine workflows that do not involve the #jcpennys (Jupyter, Conda, Pandas, Numpy, Sklearn) stack for #mlops. In particular I am not a fan of the conda installation tool (it is superfluous as [I demonstrate in the Python MLOps Template](https://github.com/nogibjj/mlops-template)) vs containerized workflows that use the Python Standard Library (Docker + pip + virtualenv) and this is a good excuse to find other solutions outside of that stack.  For example:

  * Why not also find a more performant Data Frame library, faster speed, etc.
  * Why not have a compiler?
  * Why not have a simple packaging solution?
  * Why not have a very fast computational speed?
  * Why not be able to write both for the Linux Kernel and general purpose scripting?
  * Why not see if there is a better solution than Python (which is essentially two languages scientific python and regular Python)?
  * Python is one of the least green languages in terms of energy efficiency, but [Rust is one of the best](https://greenlab.di.uminho.pt/wp-content/uploads/2017/10/sleFinal.pdf).

### Using Data (i.e. Data Science)

* StackOverflow https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages[states that #rust is on 7th year as the most loved language 87% of developers want to continue developing](https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages) in and ties with Python as the most wanted technology.  Clearly there is traction.
* According to http://www.modulecounts.com/[Modulecounts] it looks like an exponential growth curve to Rust.
![Python vs Ruby vs Rust](https://user-images.githubusercontent.com/58792/209174014-cb3d7370-d8a2-4298-847b-f1e9f9f29a69.png)

## Getting Started

This repository is a GitHub Template and you can use it to create a new repository that uses [GitHub Codespaces](https://github.com/features/codespaces).  It is pre-configured with [Rust](https://www.rust-lang.org/tools/install), [Cargo](https://crates.io/) and other useful extensions like [GitHub Copilot](https://github.com/features/copilot).

### Install and Setup

There are a few options:

* You can follow the [Official Install Guide for Rust](https://www.rust-lang.org/tools/install)
* Create a [repo with this template](https://github.com/nogibjj/rust-mlops-template)

Once you install you should check to see things work:

```rustc --version```

Other option is to run `make rust-version` which checks both the cargo and rust version.

### Rust CLI Tools Ecosystem

You can see there several tools which help you get things done in Rust:

```Makefile
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter
```

### Hello World Setup

This is an intentionally simple full end-to-end hello world example.  I used some excellent ideas from @kyclark, author of the command-line-rust book from O'Reilly [here](https://github.com/kyclark/command-line-rust/tree/master/01_hello).  You can recreate on your own following these steps

Create a project directory
* `cargo new hello`

This creates a structure you can see with `tree hello`

```bash
hello/
├── Cargo.toml
└── src
    └── main.rs
1 directory, 2 files
```

The `Cargo.toml` file is where the project is configured, i.e. if you needed to add a dependency.
The source code file has the following content in `main.rs`.  It looks a lot like Python or any other modern language and this function prints a message.

```
fn main() {
    println!("Hello, world MLOPs!");
}
```

To run the project you cd into hello and run `cargo run` i.e. `cd hello && cargo run`.  The output looks like the following:

```bash
@noahgift ➜ /workspaces/rust-mlops-template/hello (main ✗) $ cargo run
   Compiling hello v0.1.0 (/workspaces/rust-mlops-template/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/hello`
Hello, world MLOPs!
```

To run without all of the noise:  `cargo run --quiet`.
To run the binary created `./target/debug/hello`

### Run with GitHub Actions

GitHub Actions uses a `Makefile` to simplify automation

```yaml
name: Rust CI/CD Pipeline
on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]
env:
  CARGO_TERM_COLOR: always
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: update linux
      run: sudo apt update 
    - name: update Rust
      run: make install
    - name: Check Rust versions
      run: make rust-version
    - name: Format
      run: make format
    - name: Lint
      run: make lint
    - name: Test
      run: make test
```


### First Big Project:  Deduplication Command-Line Tool

I have written command-line deduplication tools in many languages so this is what I choose to build a substantial example. The general approach I use is as follows:  

* Walk the filesystem and create a checksum for each file
* If the checksum matches an existing checksum, then mark it as a duplicate file


## Language References and Tutorials

* [Comprehensive Rust Course Google](https://google.github.io/comprehensive-rust/)
* [Rust Machine Learning](https://github.com/vaaaaanquish/Awesome-Rust-MachineLearning)
* [Rust Async Book](https://github.com/rust-lang/async-book)
* [52 Weeks of Rust](https://github.com/nogibjj/52-weeks-rust)
* [Command-Line Rust Book](https://learning.oreilly.com/library/view/command-line-rust/9781098109424/ch01.html)
* [Command-Line Rust Book Source Code](https://github.com/kyclark/command-line-rust.git)

### Authoring Tools

One goal is to reduce using Notebooks in favor of lightweight markdown tools (i.e. the goal is MLOps vs interactive notebooks)

* [mdBook](https://rust-lang.github.io/mdBook/)
* [Quarto](https://quarto.org/)
