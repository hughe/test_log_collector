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

.PHONY: all build test check fmt check_fmt clippy doc clean help

## all: build the library (default)
all: build

## build: compile the crate
build:
	$(CARGO) build

## test: unit tests via nextest, plus doc tests
#
# The `--doc` line is NOT optional. nextest does not run doctests at all,
# and this crate has 7 of them — replacing a bare `cargo test` with
# nextest alone would drop them silently, with the suite still green.
test:
	$(CARGO) nextest run
	$(CARGO) test --doc

## check: the lint gate — formatting and clippy, both fatal
check: check_fmt clippy

## fmt: reformat the tree in place
#
# `fmt` REWRITES and `check_fmt` reports, the same way round in every
# component here. A gate must not edit your working tree, so `check`
# depends on the reporting one.
fmt:
	$(CARGO) fmt

## check_fmt: fail if the tree is not formatted
check_fmt:
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

## actionlint: lint this component's GitHub Actions workflows
#
# Self-contained on purpose. This file is exported to the component's
# own repository, where there is no monorepo root Makefile to fall back
# on, so `make actionlint` has to work standing alone.
#
# `wildcard` makes it a no-op rather than an error when there are no
# workflows here, so the target exists uniformly across every component
# and the root can call it without knowing which have any.
#
# WHY IT EXISTS. GitHub's workflow parser is stricter than YAML and
# everything below it is silent about the difference: on 2026-08-20 a
# comment containing the literal expression braces was valid YAML,
# passed every structural check, and was rejected outright by GitHub
# with "workflow file issue" and zero jobs.
ACTIONLINT ?= actionlint
# See the root Makefile: disabled because actionlint runs these only
# if they are on PATH, so results would vary by machine.
ACTIONLINT_FLAGS ?= -shellcheck= -pyflakes=
ACTIONLINT_FILES := $(wildcard .github/workflows/*.yml)

actionlint:
ifeq ($(ACTIONLINT_FILES),)
	@echo "actionlint: no workflows in $(CURDIR) — nothing to do"
else
	$(ACTIONLINT) $(ACTIONLINT_FLAGS) $(ACTIONLINT_FILES)
endif
.PHONY: actionlint
