#!/bin/bash

if [ $# -ne 2 ]; then
    echo "Synchronize dary_heap source with Rust standard library"
    echo "Usage: $0 [path_to_rust] [new_rev]"
    echo "  path_to_rust: Path at which Git repository of Rust is located"
    echo "  new_rev: Git revision (tag, commit, branch, etc.) to update to"
    exit 1
fi

cd "$(dirname "$0")" || { echo "Could not cd into base directory"; exit 1; }
base="$(pwd)"

rust="$1"
alloc="${rust}/library/alloc"

current_file="${base}/sync-current"
current_rev="$(<"${current_file}")" || { echo "Invalid current revision"; exit 1; }
new_rev="$2"

cd "${rust}" || { echo "Could not cd into Rust directory"; exit 1; }

current="$(git rev-parse "${current_rev}")" \
    || { echo "Current git revision (${current_rev}) not found"; exit 1; }
new="$(git rev-parse "${new_rev}")" \
    || { echo "New git revision (${new_rev}) not found"; exit 1; }

git diff "${current}" "${new}" "${alloc}/src/collections/binary_heap.rs" \
    | patch --merge=diff3 "${base}/../src/lib.rs"
git diff "${current}" "${new}" "${alloc}/tests/binary_heap.rs" \
    | patch --merge=diff3 "${base}/../tests/binary_heap.rs"

echo "${new}" > "${current_file}"
