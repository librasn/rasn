#!/usr/bin/env bash
set -e

if [ "$GITHUB_JOB" = "windows" ]; then
  rustup set auto-self-update disable
else
  rustup update
fi
rustup default $1
rustup target add $2
rustup component add rustfmt clippy
