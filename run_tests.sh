#!/usr/bin/env bash
# Runs the test gate for every lesson in order.
# Each row below: "display-name:cargo-args...".
# A lesson is considered complete when its test command exits 0.
set -u

cd "$(dirname "$0")"

lessons=(
    "01-location:-p fundamentals --no-default-features --features location"
    "02-asset:-p fundamentals --no-default-features --features asset"
    "03-instruction:-p fundamentals --no-default-features --features instruction"
    "04-cross-chain-transfer:-p execution tests::full::cross_chain_transfer"
    "05-transfer-and-transact:-p execution tests::full::transfer_and_transact"
    "06-transfer-and-swap:-p execution tests::full::transfer_and_swap"
    "07-transfer-swap-and-back:-p execution tests::full::transfer_swap_and_back"
    "08-config-asset-transactor:-p execution tests::asset_transactor"
    "09-config-barrier:-p execution tests::barrier"
    "10-config-reserves-teleports:-p execution tests::reserves_and_teleports"
)

failed=()
for entry in "${lessons[@]}"; do
    name="${entry%%:*}"
    args="${entry#*:}"
    echo
    echo "==> $name"
    # shellcheck disable=SC2086
    if cargo test $args; then
        echo "✓ $name"
    else
        echo "✗ $name"
        failed+=("$name")
    fi
done

echo
if [[ ${#failed[@]} -gt 0 ]]; then
    echo "Failing lessons: ${failed[*]}"
    exit 1
fi
echo "All lessons pass."
