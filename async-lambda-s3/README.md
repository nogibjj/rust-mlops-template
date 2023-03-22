## A lambda function that async summarizes every object in every bucket

[YouTube AWS Lambda Walkthrough](https://youtu.be/IhEMgX9Aeo4)

It requires you to deploy with a role that can access S3.
Then do `make invoke`

```bash
{
  "req_id": "46ce6303-f0f3-48cd-9d80-118538f41458",
  "msg": "Event Payload size.",
  "size": "Total size of all buckets: 114.20 GB"
}
```

![Screenshot 2023-02-12 at 10 57 07 AM](https://user-images.githubusercontent.com/58792/218321857-9c47dfaa-ea96-452c-8aec-be45063466d0.png)

### Docker version

If you want to deploy via Docker do the following command:

https://github.com/awslabs/aws-lambda-rust-runtime#25-docker

```bash
# build and package deploy-ready artifact
$ docker run --rm \
    -v ${PWD}:/code \
    -v ${HOME}/.cargo/registry:/root/.cargo/registry \
    -v ${HOME}/.cargo/git:/root/.cargo/git \
    rustserverless/lambda-rust
```

