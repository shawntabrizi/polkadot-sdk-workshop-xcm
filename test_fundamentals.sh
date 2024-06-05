
#!/bin/bash

# List of cargo features to be tested
features=(
    "location"
    "asset"
    "instruction"
    "holding"
	"xcm_executor"
	"pallet_xcm"
)

# Run cargo test commands
for feature in "${features[@]}"; do
    cargo test -p fundamentals --features $feature --no-default-features

    # Check the exit status of the previous command
    if [ $? -ne 0 ]; then
        echo "❌ Tests failed: $feature"
		echo "Exit early"
        exit 1
    fi
done

echo "✅︎ All tests pass."
