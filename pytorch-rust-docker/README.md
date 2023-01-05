### Notes


`error while loading shared libraries: libtorch_cpu.so: cannot open shared object file: No such file or directory`

* Using `rust-bert` to install pytorch here:  https://crates.io/crates/tch 
* To run: `cargo run -- resnet18.ot Walking_tiger_female.jpg`

### Docker Examples

* `docker build -t pytorch-rust-docker .`
* `docker run -it pytorch-rust-docker`
* Next inside the container run: `cargo run -- resnet18.ot Walking_tiger_female.jpg`