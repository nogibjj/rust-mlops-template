To run:

`@noahgift ➜ /workspaces/rust-mlops-template/regression-cli (main ✗) $ cargo run -- train --ratio .9
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/regression-cli train --ratio .9`
Training ratio: 0.9
intercept:  152.1586901763224
params: [0, -0, 503.58067499818077, 167.75801599203626, -0, -0, -121.6828192430516, 0, 427.9593531331433, 6.412796328606638]
z score: Ok([0.0, -0.0, 6.5939908998261245, 2.2719123245079786, -0.0, -0.0, -0.5183690897253823, 0.0, 2.2777581181031765, 0.0858408096568952], shape=[10], strides=[1], layout=CFcf (0xf), const ndim=1)
predicted variance: -0.014761955865436382`
