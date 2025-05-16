ERROR := \x1b[0;91m
INFO := \x1b[0;94m
NC := \x1b[0m

define show_help_message
	echo "Usage: make TARGET"
	echo ""
	echo "Commands:"
	grep -hE '^[A-Za-z0-9_ \-]*?:.*##.*$$' $(MAKEFILE_LIST) | \
	    awk 'BEGIN {FS = ":.*?## "}; {printf "  $(INFO)%-20s$(NC) %s\n", $$1, $$2}'
endef

define show_error_message
	echo "$(ERROR)[Error] $(1)$(NC)"
endef

PREFIX ?= /usr/local
SOURCE_DIRS := textcanvas tests

.PHONY: all
all: help

.PHONY: help
help: ## Show this help message
	@$(show_help_message)

.PHONY: clean
clean: ## Clean project files
	@rm -rf ./dist/
	@rm -rf ./var/htmlcov
	@rm -rf ./.coverage
	@rm -rf ./.ruff_cache/
	@rm -rf ./__pycache__/
	@rm -rf ./*.egg-info/
	@find $(SOURCE_DIRS) -name "__pycache__" -prune -exec rm -rf {} \;
	@find $(SOURCE_DIRS) -name "*.py[co]" -prune -exec rm -rf {} \;
	@find $(SOURCE_DIRS) -name "*.so" -prune -exec rm -rf {} \;
	@cargo clean

.PHONY: l
l: lint
.PHONY: lint
lint: ## Run various linting tools
	@pre-commit run --all-files

.PHONY: check
check: ## Most stringent checks (includes checks still in development)
	@rustup update || :
	@cargo fmt
	@cargo doc --no-deps --all-features
	@cargo check
	@cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::cargo -W clippy::complexity -W clippy::correctness -W clippy::nursery -W clippy::pedantic -W clippy::perf -W clippy::style -W clippy::suspicious -A clippy::missing_const_for_fn -A clippy::option_if_let_else -A clippy::suboptimal_flops -A clippy::while_float
	@make rust-test
	@make rust-coverage-pct
	@make python-test
	@make python-coverage-pct
	@make lint

.PHONY: rt
rt: rust-test
.PHONY: rust-test
rust-test: ## Run Rust unit tests
	@cargo test

.PHONY: lrt
lrt: light-rust-test
.PHONY: light-rust-test
light-rust-test: ## Run light Rust unit tests
	@cargo test --quiet --tests

.PHONY: pt
pt: python-test
.PHONY: python-test
python-test: ## Run Python unit tests
	@uv run python -m unittest

.PHONY: rust-doc
rust-doc: ## Build Rust documentation
	@cargo doc --all-features --document-private-items
	@echo file://$(shell pwd)/target/doc/$(shell basename $(shell pwd))/index.html

.PHONY: rc
rc: rust-coverage
.PHONY: rust-coverage
rust-coverage: ## Rust unit tests coverage report
	@cargo tarpaulin --engine Llvm --timeout 120 --skip-clean --out Html --output-dir target/ --all-features
	@echo file://$(shell pwd)/target/tarpaulin-report.html

.PHONY: rust-coverage-pct
rust-coverage-pct: ## Ensure code coverage minimum %
	@cargo tarpaulin --engine Llvm --timeout 120 --out Stdout --all-features --fail-under 99

.PHONY: pc
pc: python-coverage
.PHONY: python-coverage
python-coverage: ## Python unit tests coverage report
	@rm -rf var/htmlcov
	@uv run coverage run -m unittest
	@uv run coverage html -d var/htmlcov
	@#uv run coverage report
	@echo file://$(shell pwd)/var/htmlcov/index.html

.PHONY: python-coverage-pct
python-coverage-pct: ## Ensure code coverage of 100%
	@uv run coverage run -m unittest > /dev/null 2>&1 || :
	@uv run coverage json -q -o /dev/stdout | uv run python -c \
		'import decimal, json, sys; \
		coverage = json.loads(input(), parse_float=decimal.Decimal); \
		percent_covered = coverage["totals"]["percent_covered"]; \
		print(percent_covered); \
		sys.exit(0 if percent_covered == 100 else 1);'

# .PHONY: pp
# pp: python-profile
# .PHONY: python-profile
# python-profile: ## Profile file or module
# 	@uv run python -m cProfile -s tottime -m tests/profiling.py

%:
	@$(call show_error_message,Unknown command '$@')
	@$(show_help_message)
