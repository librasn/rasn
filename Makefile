# === Relevant CI variables ====================================
#   CROSS           - the path to cross (or cargo) executable
#   TARGET_TRIPLE   - the target triple to build/test for
#   RELEASE_BUILD   - if non-empty, build in release mode
#   RUST_CHANNEL    - the rust toolchain channel to use (default: stable)

CARGO ?= cargo
CROSS ?= $(CARGO)
RUST_CHANNEL ?= stable
TARGET_TRIPLE ?= $(shell rustc -Vv | grep host | cut -d' ' -f2)
TARGET_FLAGS := --workspace --all-targets --all-features
DOC_TARGET_FLAGS := --no-deps --target $(TARGET_TRIPLE) --release --workspace --all-features

FMT_FLAGS := --all -- --check
CLIPPY_FLAGS := $(TARGET_FLAGS) -- -D warnings

RUSTFLAGS ?= --deny warnings
RUSTDOCFLAGS ?= --deny warnings

# === Setup toolchain ==============================================
# This target is used to set up the Rust toolchain for the specified target triple.
# In local development, it will update the toolchain and add the target, and missing components.
.PHONY: setup-toolchain

setup-toolchain:
	@echo "Setting up Rust toolchain $(RUST_CHANNEL) for target $(TARGET_TRIPLE)"
	@if [ "$$GITHUB_JOB" = "windows" ]; then \
		rustup set auto-self-update disable; \
	else \
		rustup update; \
	fi
	rustup default $(RUST_CHANNEL)
	rustup target add $(TARGET_TRIPLE)
	rustup component add rustfmt clippy

.PHONY: check lint fmt build test doc all

check: fmt lint

lint:
	@echo "Running clippy..."
	$(CROSS) clippy $(CLIPPY_FLAGS)

fmt:
	@echo "Checking formatting..."
	$(CROSS) fmt $(FMT_FLAGS)

build:
	@echo "Building workspace..."
ifneq ($(RELEASE_BUILD),)
	$(CROSS) build --target $(TARGET_TRIPLE) $(TARGET_FLAGS) --release
else
	$(CROSS) build --target $(TARGET_TRIPLE) $(TARGET_FLAGS)
endif

test:
	@echo "Running all tests...(excluding doc)"
	$(CROSS) test --target $(TARGET_TRIPLE) $(TARGET_FLAGS)

# Documentation tests for all local (non-dependency) crates.
# Requires jq to parse JSON output from cargo metadata.
doc:
	@echo "Building documentation..."
	$(CROSS) doc $(DOC_TARGET_FLAGS)
	@echo "Running documentation tests for each workspace crate..."
	@for crate in $$( $(CROSS) metadata --no-deps --format-version 1 | jq -r '.packages[].name' ); do \
	  echo "Testing docs for: $$crate"; \
	  $(CROSS) test --doc -p $$crate; \
	done

release: build
	@echo "Running a possible release build with docs..."
	$(CROSS) doc $(DOC_TARGET_FLAGS) --target-dir /tmp/rasn-docs

all: check build test doc


.PHONY: ci-setup

toolchain: setup-toolchain