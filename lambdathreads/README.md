![Screenshot 2023-02-15 at 11 24 53 AM](https://user-images.githubusercontent.com/58792/219089277-65420114-6254-4cd9-94d4-f1d24b317a7a.png)

Python equivalent:  https://github.com/nogibjj/rust-tutorial/blob/main/week5/mplambda.py



## Replicate and Expand Crude Benchmark

Note, this was a very crude, buggy and fast benchmark to show the penalty of processes in a low memory environment like AWS Lambda.  To further expand this benchmark and create a cool blog post feel free to try these ideas out:

* Fix any loop bugs and replace compute with something like a fibonnici number, etc that ensure the same compute is happening.
* Dedupe Lambda: Mount EFS and create 100 1MB files of which 50 each are identical.  Using processes in Python and Threads in Rust checksum them all to deduct for duplications than sum the cost of invoking 50M times (i.e. this could be a SaaS provider's service). 
* Replicate this experiment with 100 instances in 100 AWS and do 100 threads in Rust vs 100 processes in Python and summarized cost:  https://aws.amazon.com/blogs/compute/parallel-processing-in-python-with-aws-lambda/.
* Do an ONNX prediction on 100 image files in AWS S3 or AWS EFS using 100 threads in Rust and 100 processes in Python, then sum cost at 50M invocations.

## Counter Reasons why Python might be ok in some scenarios (From Brian Tarbox, Alexa Guru)

1) many workloads are not pure cpu, in particular in my Alexa skill I call [quickchart.io](https://quickchart.io) 10x in threads to get a bunch of charts and that works great.
2) for purely cpu constrained tasks I would consider having a main lambda that invoked multiple separate helper lambdas which each ran one core and synchronized the resulting data.

**comment from Noah:  Option 2) is actually very slick (i.e you using Rust Firecracker to be your non-GIL)  The only gotcha is the idle core(s) which in theory are being wasted and your charged for it**

## References

* [Make Python Gil Optional](https://peps.python.org/pep-0703/)
* [energy usage languages](https://haslab.github.io/SAFER/scp21.pdf)
* [Note, AWS Lambda is written in Rust](https://aws.amazon.com/blogs/aws/firecracker-lightweight-virtualization-for-serverless-computing/)
* [Up to Six Cores](https://aws.amazon.com/about-aws/whats-new/2020/12/aws-lambda-supports-10gb-memory-6-vcpu-cores-lambda-functions/)
* [Relationship between Lambda and Memory](https://medium.com/@harrisaaron/multithreading-in-lambda-youll-need-to-use-this-much-memory-1ad7d257fbb3)
* [David Beazley on GIL](http://www.dabeaz.com/GIL/)
