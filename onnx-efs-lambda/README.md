## Notes

![Screenshot 2023-02-12 at 4 04 15 PM](https://user-images.githubusercontent.com/58792/218337046-e240d0e4-0406-4920-8c07-429cde193741.png)





Close, but having issue with `libonnxruntime.so.1.8.1`

```bash
/var/task/bootstrap: error while loading shared libraries: libonnxruntime.so.1.8.1: cannot open shared object file: No such file or directory
/var/task/bootstrap: error while loading shared libraries: libonnxruntime.so.1.8.1: cannot open shared object file: No such file or directory
```

one way to get around this is to setup LD_LIBRARY_PATH via AWS Lambda to "/mnt/efs"

AND VICTORY!!!!

```bash
(.venv) vscode@f19e45043f99:/workspaces/rust-mlops-template/onnx-efs-lambda$ make invoke
cargo lambda invoke --remote \
                --data-ascii '{"name": "onnx"}' \
                --output-format json \
                onnx_efs_lambda
{
  "files": "\n\n/mnt/efs/libonnxruntime.so.1.8.1\n\n/mnt/efs/foo.txt\n\n/mnt/efs/squeezenet1.0-8.onnx\n\n/mnt/efs/squeezenet1.0-12.onnx",
  "msg": "Hello, onnx!",
  "req_id": "60187c67-ec03-4f8a-b102-0576eced1492",
  "scores": [
    0.000045440578,
    0.0038458686,
    0.0001249467,
    0.0011804511,
    0.00131694
  ]
}
(.venv) vscode@f19e45043f99:/workspaces/rust-mlops-template/onnx-efs-lambda$ 
```
