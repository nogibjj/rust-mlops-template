## Notes


[Video Walkthrough](https://youtu.be/HsU873c0B-c)

![Screenshot 2023-02-12 at 4 04 15 PM](https://user-images.githubusercontent.com/58792/218337046-e240d0e4-0406-4920-8c07-429cde193741.png)

To replicate:

* A.  Create EFS and configure access point with /mnt/efs.  Also open up securitygroup to port 2049.
* B.  Configure to mount on AWS Lambda and use access point
* C.  Copy items you need in Lambda, i.e. models (squeezenet1.0-8.onnx, hugging face onnx, etc) and shared object files for example `libonnxruntime.so.1.8.1`
* D.  Set environmental variable for AWS Lambda runtime for LD_LIBRARY_PATH to point to /mnt/efs
* E.  Tell code to use model

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

### Follow up items:

* Next steps, try t5 model and create an inference that summarizes documents.  Figure out how fast inference can be on a CPU by playing around with memory/cpu combo.


#### High Memory

You can see inference time gets as low as 40 ms.

![Screenshot 2023-02-12 at 4 52 02 PM](https://user-images.githubusercontent.com/58792/218339408-5c6fd1b9-0844-406d-8a4e-0d95ecf5a230.png)

![9 3-efs-lambda-llms](https://user-images.githubusercontent.com/58792/219347763-bb6e730a-b2aa-4103-8321-93b0a452ce7c.png)




