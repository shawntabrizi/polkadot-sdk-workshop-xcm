#!/usr/bin/env bash
# Verify the reference solutions in solutions/ still compile and pass tests against the
# current polkadot-sdk pin.
#
# Mirrors every file under solutions/ onto its matching path in the workspace, runs the
# test suite, then restores the original files from an in-process backup. Works whether
# the lesson files are at starter state, half-completed, or already match solutions/.
#
# Meant for CI: whenever the SDK pin moves, this script surfaces which lesson solution
# broke so the course author can update it.
set -euo pipefail

cd "$(dirname "$0")/.."

if [[ ! -d solutions ]]; then
    echo "no solutions/ directory; nothing to check" >&2
    exit 1
fi

solution_files=()
while IFS= read -r f; do
    solution_files+=("$f")
done < <(find solutions -type f | sort)

if [[ ${#solution_files[@]} -eq 0 ]]; then
    echo "solutions/ is empty" >&2
    exit 1
fi

backup_dir="$(mktemp -d -t check-solutions.XXXXXX)"
restore() {
    echo "==> Restoring lesson files from backup"
    for src in "${solution_files[@]}"; do
        dest="${src#solutions/}"
        if [[ -f "$backup_dir/$dest" ]]; then
            mkdir -p "$(dirname "$dest")"
            cp "$backup_dir/$dest" "$dest"
        fi
    done
    rm -rf "$backup_dir"
}
trap restore EXIT

echo "==> Backing up lesson files to $backup_dir"
for src in "${solution_files[@]}"; do
    dest="${src#solutions/}"
    if [[ ! -f "$dest" ]]; then
        echo "missing target: $dest (solutions/ is out of sync with workspace)" >&2
        exit 1
    fi
    mkdir -p "$backup_dir/$(dirname "$dest")"
    cp "$dest" "$backup_dir/$dest"
done

echo "==> Swapping solutions/ into lesson paths"
for src in "${solution_files[@]}"; do
    dest="${src#solutions/}"
    cp "$src" "$dest"
done

echo "==> cargo test --workspace"
cargo test --workspace "$@"
