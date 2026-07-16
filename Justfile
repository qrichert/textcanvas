SOURCE_DIRS := "textcanvas tests"

@_default:
    just --list --list-prefix '  '

# Remove temporary files and data
clean:
    rm -rf ./dist/
    rm -rf ./var/htmlcov
    rm -rf ./.coverage
    rm -rf ./.ruff_cache/
    rm -rf ./__pycache__/
    rm -rf ./*.egg-info/
    find {{ SOURCE_DIRS }} -name "__pycache__" -prune -exec rm -rf {} \;
    find {{ SOURCE_DIRS }} -name "*.py[co]" -prune -exec rm -rf {} \;
    find {{ SOURCE_DIRS }} -name "*.so" -prune -exec rm -rf {} \;
    cargo clean

# Most stringent checks (includes checks still in development)
check:
    -rustup update
    cargo fmt
    cargo doc --no-deps --all-features
    cargo check
    cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::cargo -W clippy::complexity -W clippy::correctness -W clippy::nursery -W clippy::pedantic -W clippy::perf -W clippy::style -W clippy::suspicious -A clippy::missing_const_for_fn -A clippy::option_if_let_else -A clippy::suboptimal_flops -A clippy::while_float
    just rust-test
    just rust-coverage-pct
    just python-test
    just python-coverage-pct
    just lint

alias rt := rust-test
# Run Rust unit tests
rust-test:
    NO_COLOR= cargo test

alias lrt := light-rust-test
# Run light Rust unit tests
light-rust-test:
    cargo test --quiet --tests

alias pt := python-test
# Run Python unit tests
python-test:
    NO_COLOR= uv run python -m unittest

# Build Rust documentation
rust-doc:
    cargo doc --all-features --document-private-items
    @echo file://`pwd`/target/doc/`basename \`pwd\` | sed 's/-/_/g'`/index.html

alias rc := rust-coverage
# Rust unit tests coverage report
rust-coverage:
    cargo tarpaulin --engine Llvm --timeout 120 --skip-clean --out Html --output-dir target/ --all-features
    @echo file://`pwd`/target/tarpaulin-report.html

# Ensure Rust code coverage minimum %
rust-coverage-pct:
    cargo tarpaulin --engine Llvm --timeout 120 --out Stdout --all-features --fail-under 99

alias pc := python-coverage
# Python unit tests coverage report
python-coverage:
    rm -rf var/htmlcov
    uv run coverage run -m unittest
    uv run coverage html -d var/htmlcov
    # uv run coverage report
    @echo file://`pwd`/var/htmlcov/index.html

# Ensure Python code coverage of 100%
python-coverage-pct:
    uv run coverage run -m unittest > /dev/null 2>&1 || :
    uv run coverage json -q -o /dev/stdout | uv run python -c 'import decimal, json, sys; coverage = json.loads(input(), parse_float=decimal.Decimal); percent_covered = coverage["totals"]["percent_covered"]; print(percent_covered); sys.exit(0 if percent_covered == 100 else 1);'

alias l := lint
# Run various linting tools
lint:
    prek run --all-files
