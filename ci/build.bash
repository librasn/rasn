#!/usr/bin/env bash
# Script for building your rust projects.
set -e

source ci/common.bash

# $1 {path} = Path to cross/cargo executable
CROSS=$1
# $1 {string} = <Target Triple>
TARGET_TRIPLE=$2
# $3 {boolean} = Whether or not building for release or not.
RELEASE_BUILD=$3

required_arg "$CROSS" 'CROSS'
required_arg "$TARGET_TRIPLE" '<Target Triple>'

# Build projects. Also test that we can build the docs in crates.io.

if [ -z "$RELEASE_BUILD" ]; then
    "$CROSS" build --target "$TARGET_TRIPLE" --workspace
    "$CROSS" build --target "$TARGET_TRIPLE" --all-features --workspace
    "$CROSS" doc --target "$TARGET_TRIPLE" --release --workspace
else
    "$CROSS" build --target "$TARGET_TRIPLE" --all-features --release --workspace
    "$CROSS" doc --target "$TARGET_TRIPLE" --release --workspace
fi

