## Notes on ONNX

* Refer to guide here:  https://crates.io/crates/onnxruntime
* See models here:  https://github.com/onnx/models/tree/main/vision/classification/squeezenet
* Get runtime: `make install` will download onnx model
* Run help `cargo run -- --help`
* Invoke:  `cargo run -- infer`

## Static inclusion of model
The model is included in the binary using the `include_bytes!` macro.  This is done in the `build.rs` file.  The model is then loaded from the binary using the session builder.