### Notes

* This is a simple example of using `rust-bert` to load a pytorch model and run inference on an image.
* Using `rust-bert` to install pytorch here:  https://crates.io/crates/tch 
* To run: `cargo run -- resnet18.ot Walking_tiger_female.jpg`

### Docker Examples

You can also run inside a docker container:

* `docker build -t pytorch-rust-docker .`
* `docker run -it pytorch-rust-docker`
* Next inside the container run: `cargo run -- resnet18.ot Walking_tiger_female.jpg`