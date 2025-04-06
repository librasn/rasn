# === Relevant CI variables ====================================
#   CROSS           - the path to cross (or cargo) executable
#   TARGET_TRIPLE   - the target triple to build/test for
#   RELEASE_BUILD   - if non-empty, build in release mode
#   RUST_CHANNEL    - the rust toolchain channel to use (default: stable)

CARGO := require("cargo")
CROSS := env_var_or_default("CROSS", CARGO)
RUST_CHANNEL := "stable"
TARGET_TRIPLE := `rustc -Vv | grep host | cut -d' ' -f2`
TARGET_FLAGS := "--workspace --all-targets --all-features"
RELEASE_FLAG := if env_var_or_default("RELEASE_BUILD", "") != "" { "--release" } else { "" }
DOC_TARGET_FLAGS := "--no-deps --target " + TARGET_TRIPLE + " --release --workspace --all-features"

WORKSPACE_CRATES := shell('$1 metadata --no-deps --format-version=1 | jq -r ".packages[].name" | tr "\n" " "', CROSS)

FMT_FLAGS := "--all -- --check"
CLIPPY_FLAGS := TARGET_FLAGS + " -- -D warnings"

export RUSTFLAGS := env_var_or_default("RUSTFLAGS", "--deny warnings")
export RUSTDOCFLAGS := env_var_or_default("RUSTDOCFLAGS", "--deny warnings")

# Default entry point: Run all verification steps
default: all

# Run the complete verification suite (formatting, linting, building, testing, documentation)
all: check build test doc

# Run all code quality checks (both formatting and linting)
check: fmt lint

# Run Clippy for static code analysis with strict warning enforcement
lint:
    @echo "Running clippy..."
    {{CROSS}} clippy {{CLIPPY_FLAGS}}

# Verify code formatting compliance with default rustfmt standards
fmt:
    @echo "Checking formatting..."
    {{CROSS}} fmt {{FMT_FLAGS}}


# Build the entire workspace with proper target configuration, mode is determined by RELEASE_BUILD
build:
    @echo "Building workspace..."
    {{CROSS}} build --target {{TARGET_TRIPLE}} {{TARGET_FLAGS}} {{RELEASE_FLAG}}

# Execute the test suite across the entire workspace (excludes documentation tests)
test:
    @echo "Running all tests...(excluding doc)"
    {{CROSS}} test '--target' {{TARGET_TRIPLE}} {{TARGET_FLAGS}}

# Build documentation for the entire workspace
doc-build:
    @echo "Building documentation..."
    echo "RUSTDOCFLAGS: {{RUSTDOCFLAGS}}"
    {{CROSS}} doc {{DOC_TARGET_FLAGS}}

# Run documentation tests for each crate in the workspace (requires jq)
doc-test:
    @echo "Running documentation tests for each workspace crate..."
    for crate in {{WORKSPACE_CRATES}}; do \
        echo "Testing docs for: $crate" && \
        {{CROSS}} test --doc -p "$crate" || exit 1; \
    done

# Build documentation and run documentation tests for each crate
doc: doc-build doc-test

# Prepare a release build with complete documentation
release:
    @echo "Running a possible release build with docs..."
    {{CROSS}} doc {{DOC_TARGET_FLAGS}} '--target-dir' /tmp/rasn-docs

# Set up or update the development environment with required Rust toolchain components
toolchain:
    @echo "Setting up Rust toolchain {{RUST_CHANNEL}} for target {{TARGET_TRIPLE}}"
    if [ "${GITHUB_JOB:-}" = "windows" ]; then \
      rustup set auto-self-update disable; \
    else \
      rustup update; \
    fi
    rustup default {{RUST_CHANNEL}}
    rustup target add {{TARGET_TRIPLE}}
    rustup component add rustfmt clippy

