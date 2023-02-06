
[![Rust CI/CD Pipeline](https://github.com/noahgift/rust-mlops-template/actions/workflows/rust.yml/badge.svg)](https://github.com/noahgift/rust-mlops-template/actions/workflows/rust.yml)

# rust-mlops-template
A work in progress to build out solutions in Rust for MLOPs.
This repo is more of a cookbook style.  For a more gentle step by step guide to MLOps with Rust, please see my [lecture notes as a Rust MDBook here](https://nogibjj.github.io/rust-tutorial/).



![8-3-modern-rust-development](https://user-images.githubusercontent.com/58792/211929407-633fddb8-8f44-4d30-861c-30aaa7b481e3.png)


## Demo Hitlist (Will Solve hopefully almost every day/weekly)

* Do an [inline python example](https://github.com/fusion-engineering/inline-python)
* Train a model in PyTorch with CPU:  https://github.com/LaurentMazare/tch-rs
* Train a model in PyTorch with GPU: https://github.com/LaurentMazare/tch-rs
* Serve out ONNX with a Rust web framework like Actix
* ONNX Command-Line Tool
* Simple async network example: (network discovery or chat system)
* Rust SQLite Example
* Rust AWS Lambda
* Simple Rust GUI
* Rust Whisper Tool with [C++ Bindings](https://github.com/tazz4843/whisper-rs)
* Fast Keyword Extraction (NLP)
* Emit Random Mediterranean Meals via CLI
* Web Assembly Rust

### Advanced Aspirational Demos

* Building a database in Rust
* Building a search engine in Rust
* Building a web server in Rust
* Building a batch processing systems in Rust
* Build a command-line chat system
* Build a locate clone
* Build a load-testing tool

## Motivation

One of the key goals of this project is to determine workflows that do not involve the #jcpennys (Jupyter, Conda, Pandas, Numpy, Sklearn) stack for #mlops. In particular I am not a fan of the conda installation tool (it is superfluous as [I demonstrate in the Python MLOps Template](https://github.com/nogibjj/mlops-template)) vs containerized workflows that use the Python Standard Library (Docker + pip + virtualenv) and this is a good excuse to find other solutions outside of that stack.  For example:

  * Why not also find a more performant Data Frame library, faster speed, etc.
  * Why not have a compiler?
  * Why not have a simple packaging solution?
  * Why not have a very fast computational speed?
  * Why not be able to write both for the Linux Kernel and general purpose scripting?
  * Why not see if there is a better solution than Python (which is essentially two languages scientific python and regular Python)?
  * Python is one of the least green languages in terms of energy efficiency, but [Rust is one of the best](https://greenlab.di.uminho.pt/wp-content/uploads/2017/10/sleFinal.pdf).

### In The Beginning Was the Command-Line

What could #mlops and #datascience look like in 2023 without #jupyternotebook and "God Tools" as the center of the universe? It could be the command line. In the beginning, it was the command line, and it may be the best solution for this domain.

"What would the engineer say after you had explained your problem and enumerated all the dissatisfactions in your life? He would probably tell you that life is a very hard and complicated thing; that no interface can change that; that anyone who believes otherwise is a sucker; and that if you don't like having choices made for you, you should start making your own." -Neal Stephensen

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
To run everything locally do:  `make all` and this will format/lint/test all projects in this repository.

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
    - uses: actions/checkout@v1
    - uses: actions-rs/toolchain@v1
      with:
          toolchain: stable
          profile: minimal
          components: clippy, rustfmt
          override: true
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

To run everything locally do:  `make all`.

### Simple Marco-Polo Game

Change into `MarcoPolo` directory and run `cargo run -- play --name Marco` and you should see the following output:

```bash
Polo
```


### First Big Project:  Deduplication Command-Line Tool

I have written command-line deduplication tools in many languages so this is what I choose to build a substantial example. The general approach I use is as follows:  

1. Walk the filesystem and create a checksum for each file
2. If the checksum matches an existing checksum, then mark it as a duplicate file

*Getting Started*

* Create new project: `crate new dedupe`
* Check latest clap version: https://crates.io/crates/clap and put this version in the `Cargo.toml`
The file should look similar to this.


```toml
[package]
name = "dedupe"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = "4.0.32"

[dev-dependencies]
assert_cmd = "2"
```

* Next up make a test directory:  `mkdir tests` that is parallel to `src` and put a `cli.rs` inside
* touch a `lib.rs` file and use this for the logic then run `cargo run`
* Inside this project I also created a `Makefile` to easily do everything at once:

```Makefile
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run --quiet

all: format lint test run
```

Now as I build code, I can simply do:  `make all` and get a high quality build.

Next, let's create some test files:

```bash
echo "foo" > /tmp/one.txt
echo "foo" > /tmp/two.txt
echo "bar" > /tmp/three.txt
```

The final version works:  `cargo run -- --path /tmp`

```bash
@noahgift ➜ /workspaces/rust-mlops-template/dedupe (main ✗) $ cargo run -- --path /tmp
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/dedupe --path /tmp`
Searching path: "/tmp"
Found 5 files
Found 1 duplicates
Duplicate files: ["/tmp/two.txt", "/tmp/one.txt"]
```

Next things to complete for dedupe (in another repo):

* Switch to subcommands and create a `search` and `dedupe` subcommand
* Add better testing with sample test files
* Figure out how to release packages for multiple OS versions in GitHub

### More MLOps project ideas

* Query Hugging Face dataset cli
* Summarize News CLI
* Microservice Web Framework, trying [actix](https://actix.rs/) to start, that has a calculator API
* Microservice Web Framework deploys pre-trained model
* Descriptive Statistics on a well known dataset using https://www.pola.rs/[Polars] inside a CLI
* Train a model with PyTorch (probably via [bindings to Rust](https://github.com/LaurentMazare/tch-rs))

### Actix Microservice

* Refer to [Actix getting started guide](https://actix.rs/docs/getting-started)
* `cargo new calc && cd calc`
* add dependency to `Cargo.toml`

```toml
[package]
name = "calc"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = "4"
```

* create a `src/lib.rs` and place inside

```rust
//calculator functions

//Add two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

//Subtract two numbers
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

//Multiply two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

//Divide two numbers
pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}
```

In the `main.rs` put the following:

```rust
//Calculator Microservice
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is a calculator microservice")
}

//library add route using lib.rs
#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::add(info.0, info.1);
    HttpResponse::Ok().body(result.to_string())
}

//library subtract route using lib.rs
#[get("/subtract/{a}/{b}")]
async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::subtract(info.0, info.1);
    HttpResponse::Ok().body(result.to_string())
}

//library multiply route using lib.rs
#[get("/multiply/{a}/{b}")]
async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::multiply(info.0, info.1);
    HttpResponse::Ok().body(result.to_string())
}

//library divide route using lib.rs
#[get("/divide/{a}/{b}")]
async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::divide(info.0, info.1);
    HttpResponse::Ok().body(result.to_string())
}

//run it
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

```

Next, use a `Makefile` to ensure a simple workflow

```Makefile
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run

```

Run `make all` then test out the route by adding two numbers at /add/2/2

![actix-microservice](https://user-images.githubusercontent.com/58792/209396207-7705bd3f-db5f-410c-9805-98b449b77d07.png)

### Hugging Face Example

![hugging-face-summarize](https://user-images.githubusercontent.com/58792/209409563-d47d4cbb-c4e6-4936-936d-62f2a95a7b8f.png)

* Uses [rust-bert crate](https://crates.io/crates/rust-bert)
* Create new project `cargo new hfdemo` and cd into it:  `cd hfdemo`
* Create a new library file: `touch src/lib.rs`
* Add packages to `Cargo.toml`

```toml
[package]
name = "hfdemo"
version = "0.1.0"
edition = "2021"

[dependencies]
rust-bert = "0.19.0"
clap = {version="4.0.32", features=["derive"]}
wikipedia = "0.3.4"
```

The library code is in `lib.rs` and the `subcommands` from `clap` live in `main.rs`.  Here is the tool in action:

```bash
@noahgift ➜ /workspaces/rust-mlops-template/hfdemo (main ✗) $ cargo run sumwiki --page argentina
    Finished dev [unoptimized + debuginfo] target(s) in 4.59s
     Running `target/debug/hfdemo sumwiki --page argentina`
Argentina is a country in the southern half of South America. It covers an area of 2,780,400 km2 (1,073,500 sq mi), making it the second-largest country in South America after Brazil. It is also the fourth-largest nation in the Americas and the eighth-largest in the world.
```

### Hugging Face Q/A Example

cd into `hfqa` and run `cargo run`

```bash
```bash
cargo run --quiet -- answer --question "What is the best book from 1880 to read?" --context "The Adventures of Huckleberry Finn was released in 1880"
Answer: The Adventures of Huckleberry Finn
```
<img width="928" alt="Screenshot 2023-01-07 at 8 52 35 AM" src="https://user-images.githubusercontent.com/58792/211154766-e1061c96-7116-4f23-b5ce-79e4740c4cb8.png">


### Hugging Face Lyrics Analysis using Zero Shot Classification with SQLite

![hugging-face](https://user-images.githubusercontent.com/58792/215151354-30cadb7d-c972-479d-afc7-030bd684d4d2.png)



![Screenshot 2023-01-08 at 4 26 54 PM](https://user-images.githubusercontent.com/58792/211220043-747b0146-e1b5-4ce7-b713-04e145822dd3.png)
* [Listen to Maná - En El Muelle De San Blás](https://www.youtube.com/watch?v=teprNzF6J1I)

```
@noahgift ➜ /workspaces/rust-mlops-template/sqlite-hf (main ✗) $ cargo run --quiet -- classify
Classify lyrics.txt
rock: 0.06948944181203842
pop: 0.27735018730163574
hip hop: 0.034089818596839905
country: 0.7835917472839355
latin: 0.6906086802482605
```

Print the lyrics:
```bash
cargo run --quiet -- lyrics | less | head
```

```bash
Lyrics lyrics.txt
Uh-uh-uh-uh, uh-uh
Ella despidió a su amor
El partió en un barco en el muelle de San Blas
El juró que volvería
Y empapada en llanto, ella juró que esperaría
Miles de lunas pasaron
Y siempre ella estaba en el muelle, esperando
Muchas tardes se anidaron
Se anidaron en su pelo y en sus labios
```

#### Hugging Face GPU Translation CLI

Full working example here: [https://github.com/nogibjj/rust-pytorch-gpu-template/tree/main/translate](https://github.com/nogibjj/rust-pytorch-gpu-template/tree/main/translate)

![building-gpu-translator-hugging-face](https://user-images.githubusercontent.com/58792/215615054-8a8d6b4b-2967-4daa-bf78-9554593f0015.png)


Goal:  Translate a spanish song to english
* `cargo new translate` and cd into it
fully working GPU Hugging Face Translation CLI in Rust

run it:   `time cargo run -- translate --path lyrics.txt`

```rust
/*A library that uses Hugging Face to Translate Text
*/
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use std::fs::File;
use std::io::Read;

//build a function that reads a file and returns a string
pub fn read_file(path: String) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

//build a function that reads a file and returns an array of the lines of the file
pub fn read_file_array(path: String) -> anyhow::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let array = contents.lines().map(|s| s.to_string()).collect();
    Ok(array)
}


//build a function that reads a file and translates it
pub fn translate_file(path: String) -> anyhow::Result<()> {
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::Spanish])
        .with_target_languages(vec![Language::English])
        .create_model()?;
    let text = read_file_array(path)?;
    //pass in the text to the model
    let output = model.translate(&text, None, Language::English)?;
    for sentence in output {
        println!("{}", sentence);
    }
    Ok(())
}
```




### Polars Example

* [Example here](https://github.com/noahgift/rust-mlops-template/tree/main/polarsdf)

* cd into `polarsdf` and run `cargo run`

```bash
cargo run -- sort --rows 10
```

![Screenshot 2023-01-14 at 12 21 08 PM](https://user-images.githubusercontent.com/58792/212486612-acf4040e-1772-4dbd-98d8-4ed0aaf24b96.png)


You can see an example of how Polars can be used to sort a dataframe in a Rust cli program.  

### Parallelism

One of the outstanding features of Rust is safe, yet easy paralielism.  This project demos parallelism by benchmarking a checksum of several files.

We can see how trivial it is to speed up a program with threads:

Here is the function for the serial version:

```rust
// Create a checksum of each file and store in a HashMap if the checksum already exists, add the file to the vector of files with that checksum
pub fn checksum(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut checksums = HashMap::new();
    for file in files {
        let checksum = md5::compute(std::fs::read(&file)?);
        let checksum = format!("{:x}", checksum);
        checksums
            .entry(checksum)
            .or_insert_with(Vec::new)
            .push(file);
    }
    Ok(checksums)
}

```


cargo --quiet run -- serial
```bash
➜  parallel git:(main) ✗ time cargo --quiet run -- serial
Serial version of the program
d41d8cd98f00b204e9800998ecf8427e:
        src/data/subdir/not_utils_four-score.m4a
        src/data/not_utils_four-score.m4a
b39d1840d7beacfece35d9b45652eee1:
        src/data/utils_four-score3.m4a
        src/data/utils_four-score2.m4a
        src/data/subdir/utils_four-score3.m4a
        src/data/subdir/utils_four-score2.m4a
        src/data/subdir/utils_four-score5.m4a
        src/data/subdir/utils_four-score4.m4a
        src/data/subdir/utils_four-score.m4a
        src/data/utils_four-score5.m4a
        src/data/utils_four-score4.m4a
        src/data/utils_four-score.m4a
cargo --quiet run -- serial  0.57s user 0.02s system 81% cpu 0.729 total
``` 

vs threads

```bash
time cargo --quiet run -- parallel
Parallel version of the program
d41d8cd98f00b204e9800998ecf8427e:
        src/data/subdir/not_utils_four-score.m4a
        src/data/not_utils_four-score.m4a
b39d1840d7beacfece35d9b45652eee1:
        src/data/utils_four-score5.m4a
        src/data/subdir/utils_four-score3.m4a
        src/data/utils_four-score3.m4a
        src/data/utils_four-score.m4a
        src/data/subdir/utils_four-score.m4a
        src/data/subdir/utils_four-score2.m4a
        src/data/utils_four-score4.m4a
        src/data/utils_four-score2.m4a
        src/data/subdir/utils_four-score4.m4a
        src/data/subdir/utils_four-score5.m4a
cargo --quiet run -- parallel  0.65s user 0.04s system 262% cpu 0.262 total
```
Ok, so let's look at the code:

```rust
// Parallel version of checksum using rayon with a mutex to ensure
//that the HashMap is not accessed by multiple threads at the same time
pub fn checksum_par(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let checksums = std::sync::Mutex::new(HashMap::new());
    files.par_iter().for_each(|file| {
        let checksum = md5::compute(std::fs::read(file).unwrap());
        let checksum = format!("{:x}", checksum);
        checksums
            .lock()
            .unwrap()
            .entry(checksum)
            .or_insert_with(Vec::new)
            .push(file.to_string());
    });
    Ok(checksums.into_inner().unwrap())
}
```

The main takeaway is that we use a mutex to ensure that the HashMap is not accessed by multiple threads at the same time.  This is a very common pattern in Rust.

### Logging in Rust Example

cd into `clilog` and type: `cargo run -- --level TRACE`

<img width="933" alt="Screenshot 2023-01-02 at 8 58 38 AM" src="https://user-images.githubusercontent.com/58792/210241347-a055a3d8-0dc7-4a68-ae2a-71195e91c63e.png">


```bash
//function returns a random fruit and logs it to the console
pub fn random_fruit() -> String {
    //randomly select a fruit
    let fruit = FRUITS[rand::thread_rng().gen_range(0..5)];
    //log the fruit
    log::info!("fruit-info: {}", fruit);
    log::trace!("fruit-trace: {}", fruit);
    log::warn!("fruit-warn: {}", fruit);
    fruit.to_string()
}
```


### AWS

### Rust AWS S3 Bucket Metadata Information

* [You can get it here](https://github.com/noahgift/rust-mlops-template/tree/main/awsmetas3)

Running an optimized version was able to sum all the objects in my AWS Account about 1 second: `./target/release/awsmetas3 account-size`

![bucket summarizer](https://user-images.githubusercontent.com/58792/209720447-ebabb46f-3047-47f9-a96e-cccee0cd22f7.png)


### Rust AWS Lambda

cd into `rust-aws-lambda`

* [Rust AWS Lambda docs](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html)
* Install AWS VS Code plugin and configure it to use your AWS account.
* See GitHub repo here: https://github.com/awslabs/aws-lambda-rust-runtime#deployment

![Screenshot 2023-01-22 at 6 14 48 PM](https://user-images.githubusercontent.com/58792/213945722-cff92b84-0ac2-4c29-bb77-bef142c4257d.png)

To deploy: `make deploy` which runs: `cargo lambda build --release`

* Test inside of AWS Lambda console
* Test locally with:

```bash
cargo lambda invoke --remote \
  --data-ascii '{"command": "hi"}' \
  --output-format json \
  rust-aws-lambda
```

Result:
```bash
cargo lambda invoke --remote \
                --data-ascii '{"command": "hi"}' \
                --output-format json \
                rust-aws-lambda
{
  "msg": "Command hi executed.",
  "req_id": "1f70aff9-dc65-47be-977b-4b81bf83e7a7"
}
```




### Client-Server Example

Example lives here:  https://github.com/noahgift/rust-mlops-template/tree/main/rrgame

#### Current Status

* Client server echo working

`cargo run -- client --message "hi"`
`cargo run -- server`

<img width="822" alt="Screenshot 2022-12-27 at 7 57 24 PM" src="https://user-images.githubusercontent.com/58792/209741364-3fcdef36-7dbc-4252-b34a-fb356152554a.png">
<img width="822" alt="Screenshot 2022-12-27 at 7 57 24 PM" src="https://user-images.githubusercontent.com/58792/209741584-d96ebc91-00a8-4f7d-8fca-9f565318aa9f.png">

A bigger example lives here:  https://github.com/noahgift/rust-multiplayer-roulette-game

### Containerized Rust Applications

* [Working Containerized Rust CLI Example](https://github.com/noahgift/rust-docker-cli)

```Dockerfile
FROM rust:latest as builder
ENV APP containerized_marco_polo_cli
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
 
FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
ENTRYPOINT [ "/usr/local/bin/containerized_marco_polo_cli" ]
```

#### Containerized PyTorch Rust

cd into: `pytorch-rust-docker`


Here is the `Dockerfile`

```
FROM rust:latest as builder
ENV APP pytorch-rust-docker
WORKDIR /usr/src/$APP
COPY . .
RUN apt-get update && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .
RUN cargo build -j 6
```

* `docker build -t pytorch-rust-docker .`
* `docker run -it pytorch-rust-docker`
* Next inside the container run: `cargo run -- resnet18.ot Walking_tiger_female.jpg`


<img width="1040" alt="Screenshot 2023-01-05 at 10 12 29 AM" src="https://user-images.githubusercontent.com/58792/210814241-e86f87d4-9cb9-4ab6-90a9-5d5ddd3e545d.png">


### Tensorflow Rust Bindings

* [See tf-rust-example](https://github.com/noahgift/rust-mlops-template/tree/main/tf-rust-example)
<img width="919" alt="Screenshot 2023-01-02 at 5 59 48 PM" src="https://user-images.githubusercontent.com/58792/210283521-b9d9ddf3-54c1-4bcf-bd6b-83a4a9d9e48d.png">

```rust
/*Rust Tensorflow Hello World */

extern crate tensorflow;
use tensorflow::Tensor;

fn main() {
    let mut x = Tensor::new(&[1]);
    x[0] = 2i32;
    //print the value of x
    println!("{:?}", x[0]);
    //print the shape of x
    println!("{:?}", x.shape());
    //create a multidimensional tensor
    let mut y = Tensor::new(&[2, 2]);
    y[0] = 1i32;
    y[1] = 2i32;
    y[2] = 3i32;
    y[3] = 4i32;
    //print the value of y
    println!("{:?}", y[0]);
    //print the shape of y
    println!("{:?}", y.shape());
}
```

### PyTorch
<img width="1321" alt="Screenshot 2023-01-03 at 9 45 52 AM" src="https://user-images.githubusercontent.com/58792/210380648-0a7382f9-2d6e-4fb5-92b6-ff2a43b1e10c.png">


Pre-trained model:  cd into `pytorch-rust-example` then run:  `cargo run -- resnet18.ot Walking_tiger_female.jpg`


### PyTorch Binary with embedded pre-trained model

Using included model in binary.  See this issue about including [PyTorch with binary](https://github.com/LaurentMazare/tch-rs/issues/439)

Status:  Works, but binary cannot pickup PyTorch, so still investigating solutions.

```bash
@noahgift ➜ /workspaces/rust-mlops-template/pytorch-binary-cli (main ✗) $ cargo run -- predict --image Walking_tiger_female.jpg 
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/pytorch-binary-cli predict --image Walking_tiger_female.jpg`
Current working directory: /workspaces/rust-mlops-template/pytorch-binary-cli
Model path: ../model/resnet18.ot
Model size: 46831783
tiger, Panthera tigris                             90.42%
tiger cat                                           9.19%
zebra                                               0.21%
jaguar, panther, Panthera onca, Felis onca          0.07%
tabby, tabby cat                                    0.03%
```

![Screenshot 2023-01-22 at 4 25 48 PM](https://user-images.githubusercontent.com/58792/213941368-17ab05f3-0bca-470d-87e8-b7b6fb6d7070.png)



### Web Assembly in Rust

Cd into `hello-wasm-bindgen` and run `make install` the `make serve`

You should see something like this:

![Screenshot 2023-01-09 at 11 42 29 AM](https://user-images.githubusercontent.com/58792/211361416-3203edf5-fdb6-4b13-91b8-9313d6aaf7bd.png)



```bash
/* hello world Rust webassembly*/
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

//export the function to javascript
#[wasm_bindgen]
pub fn marco_polo(s: &str) {
    //if the string is "Marco" return "Polo"
    if s == "Marco" {
        alert("Polo");
    }
    //if the string is anything else return "Not Marco"
    else {
        alert("Not Marco");
    }
}
```

### Kmeans Example

cd into `linfa-kmeans` and run `cargo run -- cluster`

### Lasso Regression CLI
![Screenshot 2023-01-15 at 9 53 25 AM](https://user-images.githubusercontent.com/58792/212548501-51993174-c294-4d8e-91b1-5f795de903a1.png)

```bash
@noahgift ➜ /workspaces/rust-mlops-template/regression-cli (main ✗) $ cargo run -- train --ratio .9
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/regression-cli train --ratio .9`
Training ratio: 0.9
intercept:  152.1586901763224
params: [0, -0, 503.58067499818077, 167.75801599203626, -0, -0, -121.6828192430516, 0, 427.9593531331433, 6.412796328606638]
z score: Ok([0.0, -0.0, 6.5939908998261245, 2.2719123245079786, -0.0, -0.0, -0.5183690897253823, 0.0, 2.2777581181031765, 0.0858408096568952], shape=[10], strides=[1], layout=CFcf (0xf), const ndim=1)
predicted variance: -0.014761955865436382
```

### Transcription with Whisper in Rust

* Based on this https://github.com/ggerganov/whisper.cpp[CPP version]
* Rust bindings here:  https://github.com/tazz4843/whisper-rs

![Screenshot 2023-01-15 at 4 23 02 PM](https://user-images.githubusercontent.com/58792/212568247-45666de9-fb80-4120-8950-42a751eff258.png)


### Rust PyTorch Saturating GPU

* Example repo here:  https://github.com/nogibjj/rust-pytorch-gpu-template/blob/main/README.md#pytorch-rust-gpu-example
![Screenshot 2023-01-16 at 5 23 15 PM](https://user-images.githubusercontent.com/58792/212774485-af74b43e-7514-46cd-a575-5cea1ccfb45f.png)

### Rust PyTorch MNIST Saturating GPU

* Example repo here: https://github.com/nogibjj/rust-pytorch-gpu-template/blob/main/README.md#mnist-convolutional-neural-network
![Screenshot 2023-01-16 at 5 57 59 PM](https://user-images.githubusercontent.com/58792/212777601-2a2acb71-c94b-4d76-8913-702fb429bb13.png)

### Rayon Multi-threaded GPU Stress Test CLI

Stress Test CLI for both CPU and GPU PyTorch using Clap

* `cargo new stress` cd into `stress`
* To test CPU for PyTorch do: `cargo run -- cpu`
* To test GPU for PyTorch do: `cargo run -- gpu`
* To monitor CPU/Memory run `htop`
* To monitor GPU run `nvidia-smi -l 1`
* To use threaded GPU load test use: `cargo run -- tgpu`


![stress-test-cuda-gpu-with-pytorch-rust](https://user-images.githubusercontent.com/58792/215621711-c222e59b-f4e1-4fd0-8202-339b986b4090.png)


Full working example here: [https://github.com/nogibjj/rust-pytorch-gpu-template/tree/main/stress](https://github.com/nogibjj/rust-pytorch-gpu-template/tree/main/stress)

### Rust Stable Diffusion Demo

You can create it this repo for more info: https://github.com/nogibjj/rust-pytorch-gpu-template#stable-diffusion-demo

* clone this repo:  https://github.com/LaurentMazare/diffusers-rs
* Follow these setup instructions: https://github.com/LaurentMazare/diffusers-rs#clip-encoding-weights

After all the weights are downloaded run:

`cargo run --example stable-diffusion --features clap -- --prompt "A very rusty robot holding a fire torch to notebooks"`
![Screenshot 2023-01-16 at 5 57 59 PM](https://user-images.githubusercontent.com/58792/212777548-0d9619e8-ad1b-4cc9-8871-505b0b5b2345.png)

Stable Diffusion 2.1 Pegging GPU
![Screenshot 2023-01-17 at 9 30 47 AM](https://user-images.githubusercontent.com/58792/212926307-351db4bc-46ff-4e8d-8630-ce996dca65c9.png)

Rusty Robot Torching Notebooks
![sd_final](https://user-images.githubusercontent.com/58792/212926379-d460a54c-29cf-42bb-801a-29e50557369e.png)


### Randomly Select Rust Crates To Work On

cd into `rust-ideas`

`cargo run -- --help`
`cargo run -- popular --number 4`
`cargo run -- random`

```bash
@noahgift ➜ /workspaces/rust-mlops-template/rust-ideas (main ✗) $ cargo run -- random
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/rust-ideas random`
Random crate: "libc"
```

### ONNX Example

cd into `OnnxDemo` and run `make install` then `cargo run -- infer` which invokes a squeezenet model.

<img width="490" alt="Screenshot 2023-01-22 at 9 33 33 AM" src="https://user-images.githubusercontent.com/58792/213921431-e2d473c3-e76e-4884-91d3-6f92639aa324.png">

#### Sonos ONNX

Verified this works and is able to invoke runtime in a portable binary: 
[https://github.com/sonos/tract/tree/main/examples/pytorch-resnet](https://github.com/sonos/tract/tree/main/examples/pytorch-resnet)

### OpenAI

#### Switching to Rust API Example

Full working example link: [https://github.com/nogibjj/assimilate-openai/tree/main/openai](https://github.com/nogibjj/assimilate-openai/tree/main/openai)

* install Rust via Rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
Use Rust API for OpenAI (3rd party):  https://github.com/deontologician/openai-api-rust

* Create new project: `cargo new openai` and cd into it
* `make format` then `make lint` then `cargo run`

Working Example:

```bash
(.venv) @noahgift ➜ /workspaces/assimilate-openai/openai (main) $ cargo run -- complete -t "The rain in spain"
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/openai complete -t 'The rain in spain'`
Completing: The rain in spain
Loves gets you nowhere
The rain in spain
```

`lib.rs`
```rust
/*This uses Open AI to Complete Sentences */

//accets the prompt and returns the completion
pub async fn complete_prompt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_token = std::env::var("OPENAI_API_KEY")?;
    let client = openai_api::Client::new(&api_token);
    let prompt = String::from(prompt);
    let result = client.complete_prompt(prompt.as_str()).await?;
    Ok(result.choices[0].text.clone())
}

```

### Command-line Data Science with Rust (Action Items)


1. go into `dscli`
2. Figure the way to make Polars work with `linfa`
3. How can I make a kmeans cluster using Polars


### Containerized Actix Continuous Delivery to AWS App Runner

![Screenshot 2023-01-31 at 1 47 32 PM](https://user-images.githubusercontent.com/58792/215854389-e9f5fc50-1607-4b4d-9d82-180f81c44c01.png)


1. cd into `webdocker`
2. build and run container (can do via `Makefile`) or

`docker build -t fruit .`
`docker run -it --rm -p 8080:8080 fruit`

3. push to ECR
4. Tell AWS App Runner to autodeploy

### Mixing Python and Rust

#### Using Rust Module from Python

* [Pyo3](https://pyo3.rs/v0.18.0/)
Try the getting started guide:

```bash
# (replace string_sum with the desired package name)
$ mkdir string_sum
$ cd string_sum
$ python -m venv .env
$ source .env/bin/activate
$ pip install maturin
```

* Run `maturin init` and then run `maturin develop` or `make develop`
* `python`
* Run the following python code
```python
import string_sum
string_sum.sum_as_string(5, 20)
```
The output should look like this: `'25'`

#### Using Python from Rust

Follow guide here: [https://pyo3.rs/v0.18.0/](https://pyo3.rs/v0.18.0/)

* install `sudo apt-get install python3-dev`
* `cargo new pyrust` and `cd pyrust`
* tweak `Cargo.toml` and add `pyo3`
* add source code to `main.rs`
* `make run`

```bash
Hello vscode, I'm Python 3.9.2 (default, Feb 28 2021, 17:03:44) 
[GCC 10.2.1 20210110]
```

Q:  Does the target binary have Python included?
A:  Maybe.  It does appear to be able to run Python if you go to the `target`
`/workspaces/rust-mlops-template/pyrust/target/debug/pyrust`

Follow up question, can I bring this binary to a "blank" codespace with no Python and what happens!

#### Day2: Using Rust with Python

Goal:  Build a high-performance Rust module and then wrap in a Python command-line tool


### Containerized Rust Examples

* `cargo new tyrscontainer` and cd into `tyrscontainer`
* copy a `Makefile` and `Dockerfile` from `webdocker`

#### References

* [Notes for Rust Docker Container](https://hub.docker.com/_/rust)


### Build System

This build system is a bit unique because it recursives many Rust repos and tests them all!

## Language References and Tutorials

* [Comprehensive Rust Course Google](https://google.github.io/comprehensive-rust/)
* [Rust Async Book](https://github.com/rust-lang/async-book)
* [52 Weeks of Rust](https://github.com/nogibjj/52-weeks-rust)
* [Command-Line Rust Book](https://learning.oreilly.com/library/view/command-line-rust/9781098109424/ch01.html)
* [Command-Line Rust Book Source Code](https://github.com/kyclark/command-line-rust.git)
* [awesome rust](https://crates.io/crates/awesome-rust)
* [Microsoft Learn Rust](https://learn.microsoft.com/en-us/training/paths/rust-first-steps/)
* [Rust Machine Learning Book](https://github.com/rust-ml/book)

### MLOps/ML Engineering and Data Science

* [best-of-ml-rust](https://github.com/e-tornike/best-of-ml-rust)
* [Awesome-Rust-MachineLearning](https://github.com/vaaaaanquish/Awesome-Rust-MachineLearning)

### Cloud Computing

#### AWS

* [Sustainability with Rust](https://aws.amazon.com/blogs/opensource/sustainability-with-rust/?pg=devrust)
* [Rust AWS Lambda](https://github.com/awslabs/aws-lambda-rust-runtime)

#### Azure

* [Azure Rust Functions](https://learn.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other?tabs=rust%2Cmacos)

### Linux Kernel

* [Rust makes way to Linux Kernel](https://thenewstack.io/rust-in-the-linux-kernel/)

### Systems Tools

* [An extended deduplication example command-line tool](https://github.com/noahgift/rdedupe)

### Deep Learning

* [Rust bindings for the C++ api of PyTorch](https://github.com/LaurentMazare/tch-rs)
* [Rust Pytorch example](https://www.swiftdiaries.com/rust/pytorch/)
* [Rust PyTorch with Actix-Web Example](https://github.com/kykosic/actix-pytorch-example)

### Web Microservices and Serverless

* [Docker + Actix](https://github.com/patrick-fitzgerald/actix-web-docker-example)
* [Actix](https://actix.rs/docs/application)
* [AWS Lambda Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html)

### Data Frames

* [Polars](https://www.pola.rs/).  You can see an [example here](https://github.com/noahgift/rust-mlops-template/tree/main/polarsdf).


### Authoring Tools

One goal is to reduce using Notebooks in favor of lightweight markdown tools (i.e. the goal is MLOps vs interactive notebooks)

* [mdBook](https://rust-lang.github.io/mdBook/)
* [Quarto](https://quarto.org/)
* [Jupyter Rust](https://github.com/google/evcxr/blob/main/evcxr_jupyter/README.md)

### Linux Tools

* [Great example of a cross-platform tool in Rust](https://github.com/GyulyVGC/sniffnet)
* [List of Popular Rust based command-line tools](https://zaiste.net/posts/shell-commands-rust/)

### Python and Rust integration

* [Pyo3 rust binding for Python](https://github.com/PyO3/pyo3)
* [Inline Python in Rust](https://github.com/fusion-engineering/inline-python)
* [Red Hat:  Speed up Python using Rust](https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust#create_a_new_crate)

### GUI

* [egui](https://github.com/emilk/egui)
* [eframe](https://crates.io/crates/eframe)
* [iced](https://crates.io/crates/iced)

### NLP

* [Rake keyword extraction](https://docs.rs/rake/latest/rake/)

### Onnx

* [onnxruntime for Rust](https://docs.rs/onnxruntime/latest/onnxruntime/)
* [Build standalone executables with Rust and ONNX](https://github.com/sonos/tract)
* [Hugging Face discussion on Rust](https://discuss.huggingface.co/t/rust-applications/23060)

### Static Web

* [Zola](https://www.getzola.org/)

#### Pure Rust Machine Learning

* [linfa ML (a bit like sklearn)](https://github.com/rust-ml/linfa)

### Benchmarking

* [Python vs Rust](https://programming-language-benchmarks.vercel.app/python-vs-rust)
https://able.bio/haixuanTao/deep-learning-in-rust-with-gpu--26c53a7f
* [Rust is 150x (15,000%) faster, and uses about the same amount of memory compared with Python.](https://www.secondstate.io/articles/performance-rust-wasm/#:~:text=%F0%9F%92%AF%20Performance%20advantages%20of%20Rust%20and%20WebAssembly,-%E2%80%A2%202%20minutes&text=Rust%20is%202x%20(200%25),of%20memory%20compared%20with%20Python.)
* [Rust 26X faster than Python sklearn](https://www.lpalmieri.com/posts/2019-12-01-taking-ml-to-production-with-rust-a-25x-speedup/)
* [150+ million users with Rust MLOPs](https://engg.glance.com/serving-ml-at-the-speed-of-rust-748c0ca82f22)

### Testing Tools

* [Fuzz Testing Rust](https://github.com/loiclec/fuzzcheck-rs)
* [Code Coverage Tool](https://www.nytimes.com/2023/01/05/business/economy/ftc-noncompete.html)

### Containerized Rust

* [Building containerized applications with Rust](https://github.com/emk/rust-musl-builder)
* [Web Assembly + Rust containerization](https://hotg.dev/docs/hot-g/what-hotg-has-created)

### Embedded Rust

* [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)

### benchmark

https://bheisler.github.io/criterion.rs/book/criterion_rs.html

### ZSH

* [Rust ZSH modules](https://github.com/Diegovsky/zsh-module-rs)

### Time Series Rust

* [influxdb-rust](https://github.com/influxdb-rs/influxdb-rust)
* [Time Series Polars Rust](https://stackoverflow.com/questions/74668242/resample-time-series-using-polars-in-rust)

### OpenAI

* [OpenAI Rust Example](https://github.com/deontologician/openai-api-rust)
