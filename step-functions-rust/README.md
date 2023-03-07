
* create new marco polo lambda
`cargo lambda new rust-marco`

Then build, deploy and invoke: `make release` `make deploy` and `make invoke`:

```bash
(.venv) @noahgift ➜ /workspaces/rust-mlops-template/step-functions-rust/rust-marco (main) $ make invoke
cargo lambda invoke --remote \
                --data-ascii '{"name": "Marco"}' \
                --output-format json \
                rust-marco
{
  "payload": "Polo",
  "req_id": "20de1794-1055-4731-9488-7c9217ad195d"
}
```


* create new rust polo lambda
`cargo lambda new rust-polo`

