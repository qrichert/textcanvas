ERROR := \x1b[0;91m
INFO := \x1b[0;94m
NC := \x1b[0m

define show_help_message
	echo "Usage: make TARGET"
	echo ""
	echo "Commands:"
	grep -hE '^[A-Za-z0-9_ \-]*?:.*##.*$$' $(MAKEFILE_LIST) | \
	    awk 'BEGIN {FS = ":.*?## "}; {printf "  $(INFO)%-13s$(NC) %s\n", $$1, $$2}'
endef

define show_error_message
	echo "$(ERROR)[Error] $(1)$(NC)"
endef

SOURCE_DIRS := textcanvas tests

.PHONY: all
all: help

.PHONY: help
help: ## Show this help message
	@$(show_help_message)

.PHONY: clean
clean: ## Remove temporary files and data
	@rm -rf ./var/htmlcov
	@rm -rf ./.coverage
	@rm -rf ./.ruff_cache/
	@rm -rf ./__pycache__/
	@find $(SOURCE_DIRS) -name "__pycache__" -prune -exec rm -rf {} \;
	@find $(SOURCE_DIRS) -name "*.py[co]" -prune -exec rm -rf {} \;
	@find $(SOURCE_DIRS) -name "*.so" -prune -exec rm -rf {} \;
	@cargo clean

.PHONY: check
check: ## Most stringent checks (includes checks still in development)
	@rustup update || :
	@cargo fmt
	@cargo doc --no-deps --all-features
	@cargo check
	@cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::cargo -W clippy::complexity -W clippy::correctness -W clippy::nursery -W clippy::pedantic -W clippy::perf -W clippy::style -W clippy::suspicious -A clippy::missing_const_for_fn -A clippy::suboptimal_flops
	@make rust-test
	@make rust-coverage-pct

.PHONY: t
t: test
.PHONY: test
test: ## Run Python unit tests
	@python -m unittest

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

.PHONY: doc
doc: ## Build documentation
	@cargo doc

.PHONY: c
c: coverage
.PHONY: coverage
coverage: ## Unit tests coverage report
	@rm -rf var/htmlcov
	@python -m coverage run -m unittest
	@python -m coverage html -d var/htmlcov
	@#python -m coverage report
	@open var/htmlcov/index.html || xdg-open var/htmlcov/index.html || :

.PHONY: rc
rc: rust-coverage
.PHONY: rust-coverage
rust-coverage: ## Unit tests coverage report
	@cargo tarpaulin --engine Llvm --timeout 120 --out Html --output-dir target/
	@open target/tarpaulin-report.html || xdg-open target/tarpaulin-report.html || :

.PHONY: rust-coverage-pct
rust-coverage-pct: ## Ensure code coverage of 100%
	@coverage=$$(cargo tarpaulin --engine Llvm --out Stdout 2>&1); \
		percent_covered=$$(echo "$$coverage" | grep -o '^[0-9]\+\.[0-9]\+% coverage' | cut -d'%' -f1); \
		echo $$percent_covered; \
		[ $$(echo "$$percent_covered == 100" | bc -l) -eq 0 ] && exit 1; \
		exit 0

.PHONY: coverage-pct
coverage-pct: ## Ensure code coverage == 100%
	@python -m coverage run -m unittest > /dev/null 2>&1 || :
	@python -m coverage json -q -o /dev/stdout | python -c \
		'import decimal, json, sys; \
		coverage = json.loads(input(), parse_float=decimal.Decimal); \
		percent_covered = coverage["totals"]["percent_covered"]; \
		print(percent_covered); \
		sys.exit(0 if percent_covered == 100 else 1);'

#.PHONY: p
#p: profile
#.PHONY: profile
#profile: ## Profile file or module
#	@python -m cProfile -s tottime -m tests/profiling.py

.PHONY: l
l: lint
.PHONY: lint
lint: ## Run various linting tools
	@pre-commit run --all-files

.PHONY: extractdocstring
extractdocstring: ## Use docstring as README
	@. .venv/bin/activate
	@echo "# TextCanvas\n" > README.md
	@echo "[![license: MIT](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/license/mit)" >> README.md
	@echo "![GitHub Tag](https://img.shields.io/github/v/tag/qrichert/textcanvas?sort=semver&filter=*.*.*&label=release)" >> README.md
	@echo "[![crates.io](https://img.shields.io/crates/d/textcanvas?logo=rust&logoColor=white&color=orange)](https://crates.io/crates/textcanvas)" >> README.md
	@echo "[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/qrichert/textcanvas/run-tests.yml?label=tests)](https://github.com/qrichert/textcanvas/actions)" >> README.md
	@python -c \
		'import textcanvas.textcanvas; \
		docstring = textcanvas.textcanvas.__doc__; \
		docstring = docstring.replace("# ", "## "); \
		docstring = docstring.replace("TextCanvas.\n", ""); \
		lines = docstring.split("## How It Works"); \
		lines.insert(1, "## Examples\n"); \
		lines.insert(2, "\n<p align=\"center\">\n"); \
		lines.insert(3, "  <img src=\"./examples/game_of_life.png\" alt=\"Game of Life\" style=\"width: 47%;\" />\n"); \
		lines.insert(4, "  <img src=\"./examples/graph.png\" alt=\"Graph\" style=\"width: 47%;\" />\n"); \
		lines.insert(5, "</p>\n"); \
		lines.insert(6, "\n## How It Works"); \
		docstring = "".join(lines); \
		f = open("README.md", "a"); \
		f.write(docstring); \
		f.close();'
	@echo "\n## Installation\n" >> README.md
	@echo "TextCanvas provides the same API for both Python and Rust.\n" >> README.md
	@echo "The Rust version includes charting helpers which the Python version does\nnot, however.\n" >> README.md
	@echo "To install for Python, run this:\n" >> README.md
	@echo '```shell' >> README.md
	@echo "pip install git+https://github.com/qrichert/textcanvas.git" >> README.md
	@echo '```\n' >> README.md
	@echo "For Rust, run one of these:\n" >> README.md
	@echo '```shell' >> README.md
	@echo "cargo add textcanvas" >> README.md
	@echo "cargo add --git https://github.com/qrichert/textcanvas.git" >> README.md
	@echo '```' >> README.md

%:
	@$(call show_error_message,Unknown command '$@')
	@$(show_help_message)
