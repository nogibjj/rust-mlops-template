#!/usr/bin/env bash
## Format all code directories in the repostitory using cargo fmt.

for DIR in */; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo fmt -- )
done

echo "Format complete."