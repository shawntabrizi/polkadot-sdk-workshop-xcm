
#!/bin/bash

# List of cargo features to be tested
features=(
    "start"
    "relay-token"
    "other-parachain-tokens"
    "register-assets"
    "asset-hub"
    # "example"
)

# Run cargo test commands
for feature in "${features[@]}"; do
    cargo test -p simulator --features $feature --no-default-features

    # Check the exit status of the previous command
    if [ $? -ne 0 ]; then
        echo "❌ Tests failed: $feature"
		echo "Exit early"
        exit 1
    fi
done

echo "✅︎ All tests pass."
