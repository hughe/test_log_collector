# test_log_collector build entry point.
#
# A plain Cargo crate; cargo is the real build system and nothing here
# replaces it. This exists so the component answers the same four verbs
# as every other component in the SLDB family (all, test, check, clean),
# which is what lets one orchestrator drive them all.
#
# The recipes match .github/workflows/rust.yml. Keep them in step.
#
# `cargo build` / `cargo test` still work unchanged, and deleting this
# file breaks nothing — which matters because this component is exported
# to its own public repo.

CARGO ?= cargo

.PHONY: all build test check fmt clippy doc clean help

## all: build the library (default)
all: build

## build: compile the crate
build:
	$(CARGO) build

## test: run the tests
test:
	$(CARGO) test

## check: the lint gate — formatting and clippy, both fatal
check: fmt clippy

## fmt: fail if the tree is not formatted
fmt:
	$(CARGO) fmt -- --check

## clippy: fail on any warning
#
# Deliberately NOT --all-targets, which is what failpoint uses. Its CI
# runs the plain form, and --all-targets currently reports 3 errors in
# tests/unit_tests.rs (two empty-string writeln!, one unhandled written
# amount). Tightening this belongs with fixing those, not with the move
# into the monorepo.
clippy:
	$(CARGO) clippy -- -D warnings

## doc: build the documentation
doc:
	$(CARGO) doc --no-deps

## clean: remove build artefacts
clean:
	$(CARGO) clean

## help: list targets
help:
	@grep -hE '^## [a-z]' $(MAKEFILE_LIST) | sed 's/^## /  /'
