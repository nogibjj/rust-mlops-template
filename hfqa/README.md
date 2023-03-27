```bash
@noahgift ➜ /workspaces/rust-mlops-template/hfqa (main) $ cargo run -- question --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/hfqa question --help`
Usage: hfqa question --qname <QNAME> <CONTEXT>

Arguments:
  <CONTEXT>  

Options:
  -q, --qname <QNAME>  
  -h, --help           Print help
  -V, --version        Print version
  ```

  An example of it working:

  ```cargo run -- question -q "What is my name" "I am a robot in the future"```


##  Action Items

Potential Solution for Static Linking?

```
While building PyTorch from source, make sure to enable static linking by setting the USE_STATIC_DISPATCH and BUILD_SHARED_LIBS options to ON. You can use the following CMake options:

-DBUILD_SHARED_LIBS=OFF -DUSE_STATIC_DISPATCH=ON
```

Compile your Rust code with the PyTorch static libraries:
In your Rust project, create a build.rs file to specify the location of the PyTorch static libraries and include files. Replace /path/to/pytorch with the actual path to your PyTorch source directory.

```rust
// build.rs
use std::env;

fn main() {
    let pytorch_path = "/path/to/pytorch";
    let torch_lib_path = format!("{}/torch/lib", pytorch_path);
    let include_path = format!("{}/torch/include", pytorch_path);

    println!("cargo:rustc-link-search=native={}", torch_lib_path);
    println!("cargo:rustc-link-lib=static=torch");
    println!("cargo:rustc-link-lib=static=torch_cpu");
    println!("cargo:rustc-link-lib=static=c10");
    env::set_var("C_INCLUDE_PATH", &include_path);
}
```

Then test it

```rust
use tch::nn;
use tch::Tensor;

fn main() {
    let vs = nn::VarStore::new(tch::Device::Cpu);
    let linear = nn::linear(vs.root(), 5, 2, Default::default());
    let input = Tensor::randn(&[3, 5], (
```



