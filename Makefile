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

.PHONY: t
t: test
.PHONY: test
test: ## Run unit tests
	@python -m unittest

.PHONY: c
c: coverage
.PHONY: coverage
coverage: ## Unit tests coverage report
	@rm -rf var/htmlcov
	@python -m coverage run -m unittest
	@python -m coverage html -d var/htmlcov
	@#python -m coverage report
	@open var/htmlcov/index.html || xdg-open var/htmlcov/index.html || :

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
	@python -c \
		'import textcanvas.textcanvas; \
		docstring = textcanvas.textcanvas.__doc__; \
		docstring = docstring.replace("TextCanvas.", "# TextCanvas"); \
		docstring = docstring.replace("How It Works:", "## How It Works"); \
		docstring = docstring.replace("See Also:", "## See Also"); \
		f = open("README.md", "w"); \
		f.write(docstring); \
		f.close();'
	@echo "\n## Installation\n" >> README.md
	@echo '```shell' >> README.md
	@echo "pip install git+https://github.com/qrichert/textcanvas.git" >> README.md
	@echo '```' >> README.md

%:
	@$(call show_error_message,Unknown command '$@')
	@$(show_help_message)
