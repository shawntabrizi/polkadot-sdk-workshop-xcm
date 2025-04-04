#!/bin/bash

# Run cargo test commands
cargo test -p execution

# Check the exit status of the previous command
if [ $? -ne 0 ]; then
    echo "❌ Tests failed: $feature"
    exit 1
fi

echo "✅︎ All tests pass."
