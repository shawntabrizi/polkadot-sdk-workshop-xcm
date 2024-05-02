
#!/bin/bash

# List of cargo features to be tested
features=(
    "fundamentals,location"
    "fundamentals,asset"
    "fundamentals,instruction"
	"fundamentals,xcm_executor"
	"fundamentals,pallet_xcm"
)

# Run cargo test commands
for feature in "${features[@]}"; do
    cargo test --features $feature --no-default-features

    # Check the exit status of the previous command
    if [ $? -ne 0 ]; then
        echo "❌ Tests failed: $feature"
		echo "Exit early"
        exit 1
    fi
done

echo "✅︎ All tests pass."
