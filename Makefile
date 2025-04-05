CARGO ?= cargo
CROSS ?= $(CARGO)
TARGET_TRIPLE ?= $(shell rustc -Vv | grep host | cut -d' ' -f2)
RUST_CHANNEL ?= stable
TARGET_FLAGS := --workspace --all-targets --all-features
FMT_FLAGS := --all -- --check
CLIPPY_FLAGS := $(TARGET_FLAGS) -- -D warnings
RUSTDOCFLAGS ?= --deny warnings

# === Helper functions ===============================================
# Helper: Check that required variables are set.
# Usage: $(call require,VARIABLE_NAME)
define require
	@if [ -z "$($(1))" ]; then \
	  echo "Error: Variable '$(1)' is not set."; \
	  exit 1; \
	fi
endef

# === Setup toolchain ==============================================
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

# === Local targets ==================================================
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
	$(CROSS) build $(TARGET_FLAGS)

test:
	@echo "Running all tests...(excluding doc)"
	$(CROSS) test $(TARGET_FLAGS)



# Documentation tests for all local (non-dependency) crates.
# Requires jq to parse JSON output from cargo metadata.
doc:
	@echo "Running documentation tests for each workspace crate..."
	@for crate in $$( $(CROSS) metadata --no-deps --format-version 1 | jq -r '.packages[].name' ); do \
	  echo "Testing docs for: $$crate"; \
	  $(CROSS) test --doc -p $$crate; \
	done

all: check build test doc

# === CI targets ====================================
# The following CI targets abstract the original bash scripts functionality
# They assume the following variables are defined:
#   CROSS           - the path to cross (or cargo) executable
#   TARGET_TRIPLE   - the target triple to build/test for
#   RELEASE_BUILD   - if non-empty, build in release mode

.PHONY: ci-setup ci-build ci-doc ci-test ci-all

ci-setup: setup-toolchain

ci-build:
	@echo "Running CI build..."
	$(call require,CROSS)
	$(call require,TARGET_TRIPLE)
ifneq ($(RELEASE_BUILD),)
	$(CROSS) build --target $(TARGET_TRIPLE) $(TARGET_FLAGS) --release
	$(CROSS) doc --no-deps --target $(TARGET_TRIPLE) --release $(TARGET_FLAGS) --target-dir /tmp/rasn-docs
else
	$(CROSS) build --target $(TARGET_TRIPLE) $(TARGET_FLAGS)
	$(CROSS) doc --no-deps --target $(TARGET_TRIPLE) --release $(TARGET_FLAGS) --target-dir /tmp/rasn-docs
endif

ci-doc:
	@echo "Running CI documentation build..."
	$(call require,CROSS)
	RUSTDOCFLAGS='--deny warnings' $(CROSS) doc $(TARGET_FLAGS) --no-deps

ci-test:
	@echo "Running CI tests..."
	$(call require,CROSS)
	$(call require,TARGET_TRIPLE)
	$(CROSS) test --target $(TARGET_TRIPLE) $(TARGET_FLAGS)

ci-all: fmt lint ci-build ci-doc ci-test
