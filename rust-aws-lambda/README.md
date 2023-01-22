### Notes

* [Rust AWS Lambda docs](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html)
* Install AWS VS Code plugin and configure it to use your AWS account.
* See GitHub repo here: https://github.com/awslabs/aws-lambda-rust-runtime#deployment


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
