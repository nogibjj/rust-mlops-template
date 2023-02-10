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

