#!/bin/bash

# Run cargo test
cargo test -p execution

# Check the exit status of the previous command
if [ $? -ne 0 ]; then
    echo "❌ Tests failed."
    exit 1
fi

echo "✅︎ All tests pass."
