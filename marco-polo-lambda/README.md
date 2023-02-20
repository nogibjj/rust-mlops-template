### Steps to run

Step 1:  [Install Cargo Lambda](https://www.cargo-lambda.info)  


Step 2:

* `make format` to format code
* `make lint` to lint
* `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
* `make deploy` which is this`cargo lambda deploy`

```Working demo
(.venv) @noahgift ➜ /workspaces/rust-mlops-template/marco-polo-lambda (main) $ make invoke
cargo lambda invoke --remote \
                --data-ascii '{"name": "Marco"}' \
                --output-format json \
                marco-polo-lambda
{
  "msg": "Marco says Polo",
  "req_id": "abc67e2b-a3aa-47fa-98fb-d07eb627577e"
}
```
