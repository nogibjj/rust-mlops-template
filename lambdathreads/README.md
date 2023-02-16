![Screenshot 2023-02-15 at 11 24 53 AM](https://user-images.githubusercontent.com/58792/219089277-65420114-6254-4cd9-94d4-f1d24b317a7a.png)

Python equivalent:  https://github.com/nogibjj/rust-tutorial/blob/main/week5/mplambda.py



## Replicate and Expand Crude Benchmark

Note, this was a very crude, buggy and fast benchmark to show the penalty of processes in a low memory environment like AWS Lambda.  To further expand this benchmark and create a cool blog post feel free to try these ideas out:

* Dedupe Lambda: Mount EFS and create 100 1MB files of which 50 each are identical.  Using processes in Python and Threads in Rust checksum them all to deduct for duplications than sum the cost of invoking 50M times (i.e. this could be a SaaS provider's service). 
* Replicate this experiment with 100 instances in 100 AWS and do 100 threads in Rust vs 100 processes in Python and summarized cost:  https://aws.amazon.com/blogs/compute/parallel-processing-in-python-with-aws-lambda/.
* Do an ONNX prediction on 100 image files in AWS S3 or AWS EFS using 100 threads in Rust and 100 processes in Python, then sum cost at 50M invocations.


## References

* [Make Python Gil Optional](https://peps.python.org/pep-0703/)
* [energy usage languages](https://haslab.github.io/SAFER/scp21.pdf)
* [Note, AWS Lambda is written in Rust](https://aws.amazon.com/blogs/aws/firecracker-lightweight-virtualization-for-serverless-computing/)
