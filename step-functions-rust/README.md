
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


![Screenshot 2023-03-07 at 12 06 12 PM](https://user-images.githubusercontent.com/58792/223496628-e6e6e221-68e4-4930-b1bd-001ebbbb4235.png)

![Screenshot 2023-03-07 at 12 08 48 PM](https://user-images.githubusercontent.com/58792/223496705-08ead2cb-70a0-47da-8fad-e558c3769217.png)

<iframe width="560" height="315" src="https://www.youtube.com/embed/2UktR8XSCE0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

Walkthrough:  https://www.youtube.com/watch?v=2UktR8XSCE0
