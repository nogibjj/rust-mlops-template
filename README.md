# rust-mlops-template
A work in progress to build out solutions in Rust for MLOPs


## Motivation

One of the key goals of this project is to determine workflows that do not involve the #jcpennys (Jupyter, Conda, Pandas, Numpy, Sklearn) stack for #mlops. In particular I am not a fan of the conda installation tool (it is superfluous as [I demonstrate in the Python MLOps Template](https://github.com/nogibjj/mlops-template) vs containerized workflows that use the Python Standard Library (Docker + pip + virtualenv) and this is a good excuse to find other solutions outside of that stack.  For example:

  * Why not also find a more performant Data Frame library, faster speed, etc.
  * Why not have a compiler?
  * Why not have a simple packaging solution?
  * Why not have a very fast computational speed?
  * Why not be able to write both for the Linux Kernel and general purpose scripting?
  * Why not see if there is a better solution than Python (which is essentially two languages scientific python and regular Python)?

### Using Data (i.e. Data Science)

* StackOverflow https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages[states that #rust is on 7th year as the most loved language 87% of developers want to continue developing](https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages) in and ties with Python as the most wanted technology.  Clearly there is traction.
* According to http://www.modulecounts.com/[Modulecounts] it looks like an exponential growth curve to Rust.
![Python vs Ruby vs Rust](https://user-images.githubusercontent.com/58792/209174014-cb3d7370-d8a2-4298-847b-f1e9f9f29a69.png)

### Getting Started

This repository is a GitHub Template and you can use it to create a new repository that uses [GitHub Codespaces](https://github.com/features/codespaces).  It is pre-configured with [Rust](https://www.rust-lang.org/tools/install), [Cargo](https://crates.io/) and other useful extensions like [GitHub Copilot](https://github.com/features/copilot).

#### Install

There are a few options:

* You can follow the [Official Install Guide for Rust](https://www.rust-lang.org/tools/install)
* Create a [repo with this template](https://github.com/nogibjj/rust-mlops-template)

#### First Project:  Deduplication Command-Line Tool

I have written command-line deduplication tools in many languages so this is what I choose to build a substantial example. The general approach I use is as follows:  

* Walk the filesystem and create a checksum for each file
* If the checksum matches an existing checksum, then mark it as a duplicate file


## Language References and Tutorials

* [Comprehensive Rust Course Google](https://google.github.io/comprehensive-rust/)
* [Rust Machine Learning](https://github.com/vaaaaanquish/Awesome-Rust-MachineLearning)
* [Rust Async Book](https://github.com/rust-lang/async-book)
* [52 Weeks of Rust](https://github.com/nogibjj/52-weeks-rust)
* [Command-Line Rust](https://learning.oreilly.com/library/view/command-line-rust/9781098109424/ch01.html)


### Authoring Tools

* [mdBook](https://rust-lang.github.io/mdBook/)
