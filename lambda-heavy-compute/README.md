Rust over 400 times faster

<img width="1283" alt="Screenshot 2023-02-10 at 3 44 35 PM" src="https://user-images.githubusercontent.com/58792/218194202-848ff0d8-140b-419e-9b7c-164c233ae91e.png">




A rust version of the following

```python
def lambda_handler(event, context):
    result = ''
    for i in range(1, 11):
        # Perform a heavy computation
        computation = sum(range(1000000))

        result += 'Iteration {}: {}\n'.format(i, computation)

    return result
```

